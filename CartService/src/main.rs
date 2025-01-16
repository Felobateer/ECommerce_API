use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json, IntoResponse},
    routing::get,
    Router, Server,
};
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::MySqlPool;
use std::{env, net::SocketAddr};
use uuid::Uuid;

mod db;
mod services;

#[derive(Clone)]
struct AppState {
    db_pool: MySqlPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a database connection pool
    let db_pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Share application state
    let app_state = AppState { db_pool };

    // Build the router
    let app = Router::new()
        .route("/user/:id", get(get_user))
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

async fn get_user(
    Path(user_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let result = services::user_service::get_user_by_id(&app_state.db_pool, user_id).await;

    match result {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(err) => {
            eprintln!("Error fetching user: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
