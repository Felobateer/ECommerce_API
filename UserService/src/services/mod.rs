use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::Utc;
use crate::db::models::User;
use crate::utils::{password, validation, jwt};
use actix_web::web;

pub struct UserService {
    pub db_pool: MySqlPool,
    pub jwt_secret: String, // Secret key for JWT generation
}

impl UserService {
    /// Create a new user in the database.
    pub async fn create_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
        phone_number: Option<&str>,
        secondary_email: Option<&str>,
        mailing_address: Option<&str>,
        secondary_address: Option<&str>,
    ) -> Result<User, String> {
        // Validate email
        if !validation::validate_email(email) {
            return Err("Invalid email format".to_string());
        }

        // Hash the password
        let password_hash = password::hash_password(password)
            .map_err(|e| format!("Failed to hash password: {:?}", e))?;

        // Create user instance
        let new_user = User {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            password_hash: password_hash.clone(),
            phone_number: phone_number.map(String::from),
            secondary_email: secondary_email.map(String::from),
            mailing_address: mailing_address.map(String::from),
            secondary_address: secondary_address.map(String::from),
            is_active: true,
            role: "customer".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Insert into the database
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, first_name, last_name, email, password_hash, phone_number,
                secondary_email, mailing_address, secondary_address, is_active,
                role, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            new_user.id,
            new_user.first_name,
            new_user.last_name,
            new_user.email,
            new_user.password_hash,
            new_user.phone_number,
            new_user.secondary_email,
            new_user.mailing_address,
            new_user.secondary_address,
            new_user.is_active,
            new_user.role,
            new_user.created_at,
            new_user.updated_at,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to insert user: {:?}", e))?;

        Ok(new_user)
    }

    /// Authenticate a user and return a JWT token.
    pub async fn authenticate_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<String, String> {
        // Fetch the user by email
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = ? AND is_active = true
            "#,
            email
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to fetch user: {:?}", e))?
        .ok_or("User not found or inactive")?;

        // Verify the password
        if !password::verify_password(&user.password_hash, password)
            .map_err(|e| format!("Failed to verify password: {:?}", e))?
        {
            return Err("Invalid password".to_string());
        }

        // Generate a JWT token
        let token = jwt::generate_token(&user.id.to_string(), &self.jwt_secret, 3600)
            .map_err(|e| format!("Failed to generate token: {:?}", e))?;

        Ok(token)
    }

    /// Fetch a user by ID.
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, String> {
        let result = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to fetch user: {:?}", e))?;

        Ok(result)
    }

    /// Update user information.
    pub async fn update_user(
        &self,
        user_id: Uuid,
        first_name: Option<&str>,
        last_name: Option<&str>,
        phone_number: Option<&str>,
        mailing_address: Option<&str>,
    ) -> Result<Option<User>, String> {
        let existing_user = self.get_user_by_id(user_id).await?;

        if let Some(mut user) = existing_user {
            user.first_name = first_name.map(String::from).unwrap_or(user.first_name);
            user.last_name = last_name.map(String::from).unwrap_or(user.last_name);
            user.phone_number = phone_number.map(String::from).or(user.phone_number);
            user.mailing_address = mailing_address.map(String::from).or(user.mailing_address);
            user.updated_at = Utc::now();

            sqlx::query!(
                r#"
                UPDATE users SET
                    first_name = ?,
                    last_name = ?,
                    phone_number = ?,
                    mailing_address = ?,
                    updated_at = ?
                WHERE id = ?
                "#,
                user.first_name,
                user.last_name,
                user.phone_number,
                user.mailing_address,
                user.updated_at,
                user.id
            )
            .execute(&self.db_pool)
            .await
            .map_err(|e| format!("Failed to update user: {:?}", e))?;

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Deactivate a user (soft delete).
    pub async fn deactivate_user(&self, user_id: Uuid) -> Result<bool, String> {
        let result = sqlx::query!(
            r#"
            UPDATE users SET is_active = false, updated_at = ? WHERE id = ?
            "#,
            Utc::now(),
            user_id
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to deactivate user: {:?}", e))?;

        Ok(result.rows_affected() > 0)
    }

    /// Invalidate the JWT token (by adding it to a blacklist or marking it invalid).
    pub async fn invalidate_token(&self, token: &str) -> Result<(), String> {
        // Add the token to a blacklist table with an expiration timestamp.
        sqlx::query!(
            r#"
            INSERT INTO token_blacklist (token, expires_at)
            VALUES (?, ?)
            "#,
            token,
            Utc::now() // Add your token expiration time here
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to invalidate token: {:?}", e))?;

        Ok(())
    }

    /// Check if a token is blacklisted.
    pub async fn is_token_blacklisted(&self, token: &str) -> Result<bool, String> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM token_blacklist WHERE token = ? AND expires_at > ?
            "#,
            token,
            Utc::now()
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to check token blacklist: {:?}", e))?;

        Ok(result.count > 0)
    }

    /// Verify a JWT token and return the associated user ID if valid.
    pub async fn verify_token(&self, token: &str) -> Result<Uuid, String> {
        // Check if the token is blacklisted
        if self.is_token_blacklisted(token).await? {
            return Err("Token is invalidated".to_string());
        }

        // Decode and validate the token
        let claims = jwt::validate_token(token, &self.jwt_secret)
            .map_err(|e| format!("Failed to validate token: {:?}", e))?;

        // Extract user ID from claims
        let user_id = claims.sub.parse::<Uuid>()
            .map_err(|_| "Invalid user ID in token".to_string())?;

        Ok(user_id)
    }
}
