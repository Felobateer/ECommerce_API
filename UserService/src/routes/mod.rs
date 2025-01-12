use actix_web::{web, HttpResponse, Responder};
use crate::handlers::{login_user, register_user, logout_user, authenticate_user, deactivate_user, edit_user};
use crate::services::user_service::UserService;
use crate::models::{LoginRequest, RegisterUserRequest, LoginResponse, DeactivateUserRequest, EditUserRequest};
use crate::utils::jwt::{generate_jwt, check_token_blacklist}; // Ensure `check_token_blacklist` is imported

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    // Public routes (no authentication required)
    cfg.route("/login", web::post().to(login_user))      // POST /login
       .route("/register", web::post().to(register_user)); // POST /register
    
    // Protected routes (authentication required)
    cfg.route("/logout", web::post().to(logout_user))        // POST /logout
       .route("/authenticate", web::get().to(authenticate_user))  // GET /authenticate
       .route("/deactivate", web::post().to(deactivate_user))     // POST /deactivate
       .route("/edit", web::put().to(edit_user))                 // PUT /edit
       .wrap_fn(check_token_blacklist); // Apply authentication middleware
}

pub async fn login_user(
    user_service: web::Data<UserService>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    match user_service.authenticate_user(&credentials.email, &credentials.password).await {
        Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
        Err(_) => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}

pub async fn register_user(
    user_service: web::Data<UserService>,
    new_user: web::Json<RegisterUserRequest>,
) -> impl Responder {
    match user_service.create_user(
        &new_user.first_name,
        &new_user.last_name,
        &new_user.email,
        &new_user.password,
        new_user.phone_number.as_deref(),
        new_user.secondary_email.as_deref(),
        new_user.mailing_address.as_deref(),
        new_user.secondary_address.as_deref(),
    ).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::BadRequest().json("Failed to register user"),
    }
}

pub async fn logout_user(
    user_service: web::Data<UserService>,
    token: web::Json<String>,
) -> impl Responder {
    match user_service.invalidate_token(&token).await {
        Ok(_) => HttpResponse::Ok().json("Logged out successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to log out"),
    }
}

pub async fn authenticate_user(
    user_service: web::Data<UserService>,
    token: web::Json<String>,
) -> impl Responder {
    match user_service.verify_token(&token).await {
        Ok(_) => HttpResponse::Ok().json("User is authenticated"),
        Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

pub async fn deactivate_user(
    user_service: web::Data<UserService>,
    credentials: web::Json<DeactivateUserRequest>,
) -> impl Responder {
    match user_service.deactivate_user(&credentials.email).await {
        Ok(_) => HttpResponse::Ok().json("User deactivated successfully"),
        Err(_) => HttpResponse::BadRequest().json("Failed to deactivate user"),
    }
}

pub async fn edit_user(
    user_service: web::Data<UserService>,
    user_data: web::Json<EditUserRequest>,
) -> impl Responder {
    match user_service.edit_user(
        &user_data.id,
        user_data.first_name.as_deref(),
        user_data.last_name.as_deref(),
        user_data.email.as_deref(),
        user_data.phone_number.as_deref(),
        user_data.secondary_email.as_deref(),
        user_data.mailing_address.as_deref(),
        user_data.secondary_address.as_deref(),
    ).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().json("Failed to update user"),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Adding user-related routes
    cfg.service(web::resource("/users").route(web::post().to(register_user))); // POST /users to register user
    cfg.service(web::resource("/users/{id}").route(web::get().to(get_user)));   // GET /users/{id} to get user
}
