use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;
use dotenvy::dotenv;
use std::env;
use crate::services::user_service::UserService;

mod db;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a database connection pool
    let db_pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let user_service = web::Data::new(UserService { db_pool });

    HttpServer::new(move || {
        App::new()
            .app_data(user_service.clone()) // Inject service
            .route("/user/{id}", web::get().to(get_user)) // Example route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_user(
    user_service: web::Data<UserService>,
    path: web::Path<Uuid>,
) -> actix_web::Result<impl actix_web::Responder> {
    let user_id = path.into_inner();

    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(web::Json(user)),
        Ok(None) => Err(actix_web::error::ErrorNotFound("User not found")),
        Err(err) => {
            eprintln!("Error fetching user: {:?}", err);
            Err(actix_web::error::ErrorInternalServerError("Database error"))
        }
    }
}
