use actix_web::{web, HttpResponse, Result};
use crate::services::user_service::UserService;
use crate::utils::validation;
use crate::models::{LoginRequest, LoginResponse, RegisterUserRequest};
use log::{error, info};
use chrono::Utc;
use sqlx::MySqlPool;

pub async fn login_user(
    user_service: web::Data<UserService>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    if !validation::validate_email(&credentials.email) {
        return Ok(HttpResponse::BadRequest().json("Invalid email format"));
    }

    match user_service.authenticate_user(&credentials.email, &credentials.password).await {
        Ok(token) => Ok(HttpResponse::Ok().json(LoginResponse { token })),
        Err(err) => {
            error!("Login failed: {}", err);
            Ok(HttpResponse::Unauthorized().json("Invalid email or password"))
        }
    }
}

pub async fn register_user(
    user_service: web::Data<UserService>,
    new_user: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse> {
    // Validate email
    if !validation::validate_email(&new_user.email) {
        return Ok(HttpResponse::BadRequest().json("Invalid email format"));
    }

    // Validate other fields (optional, as `UserService` may already handle them)

    match user_service.create_user(
        &new_user.first_name,
        &new_user.last_name,
        &new_user.email,
        &new_user.password,
        new_user.phone_number.as_deref(),
        new_user.secondary_email.as_deref(),
        new_user.mailing_address.as_deref(),
        new_user.secondary_address.as_deref(),
    )
    .await
    {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(err) => {
            error!("User registration failed: {}", err);
            Ok(HttpResponse::BadRequest().json(format!("Error: {}", err)))
        }
    }
}

pub async fn logout_user(
    user_service: web::Data<UserService>,
    token: web::Header<String>, // JWT Token from the Authorization header
) -> Result<HttpResponse> {
    match user_service.invalidate_token(&token.into_inner()).await {
        Ok(_) => {
            info!("Token invalidated successfully");
            Ok(HttpResponse::Ok().json("Logout successful"))
        }
        Err(err) => {
            info!("Logout failed: {}", err);
            Ok(HttpResponse::InternalServerError().json("Failed to log out"))
        }
    }
}

pub async fn authenticate_user(
    user_service: web::Data<UserService>,
    token: web::Header<String>, // JWT Token from the Authorization header
) -> Result<HttpResponse> {
    match user_service.verify_token(&token.into_inner()).await {
        Ok(user_id) => Ok(HttpResponse::Ok().json(format!("Authenticated user: {}", user_id))),
        Err(err) => {
            info!("Authentication failed: {}", err);
            Ok(HttpResponse::Unauthorized().json("Unauthorized"))
        }
    }
}