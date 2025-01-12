use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::Utc;
use crate::db::models::User;

pub struct UserService {
    pub db_pool: MySqlPool,
}

impl UserService {
    /// Create a new user in the database.
    pub async fn create_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password_hash: &str,
        phone_number: Option<&str>,
        secondary_email: Option<&str>,
        mailing_address: Option<&str>,
        secondary_address: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let new_user = User {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            phone_number: phone_number.map(String::from),
            secondary_email: secondary_email.map(String::from),
            mailing_address: mailing_address.map(String::from),
            secondary_address: secondary_address.map(String::from),
            is_active: true,
            role: "customer".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

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
        .await?;

        Ok(new_user)
    }

    /// Fetch a user by ID.
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await?;

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
    ) -> Result<Option<User>, sqlx::Error> {
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
            .await?;

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Deactivate a user (soft delete).
    pub async fn deactivate_user(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users SET is_active = false, updated_at = ? WHERE id = ?
            "#,
            Utc::now(),
            user_id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
