use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
    extract::State,
};
use futures::future::{BoxFuture, FutureExt};
use sqlx::MySqlPool;
use chrono::Utc;
use tracing::info;
use crate::services::user_service::authenticate;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}

pub struct LoggerMiddleware;

impl LoggerMiddleware {
    pub async fn handle<B>(
        req: Request<B>,
        next: Next<B>,
        State(app_state): State<AppState>,
    ) -> Response {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        // Log the request details
        info!("Incoming request: {} {}", method, path);

        // Check for authentication token (if needed)
        if let Some(auth_header) = req.headers().get("Authorization") {
            let token = auth_header.to_str().unwrap_or_default().trim_start_matches("Bearer ");

            match authenticate(&app_state.db_pool, token).await {
                Ok(is_valid) if is_valid => {
                    // Proceed if token is valid
                    let response = next.run(req).await;

                    // Log the response status
                    let status = response.status();
                    info!("Response status: {}", status);

                    return response;
                }
                Ok(_) => {
                    info!("Authentication failed for token: {}", token);
                    return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
                }
                Err(err) => {
                    eprintln!("Error during authentication: {:?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response();
                }
            }
        }

        // If no Authorization header is present
        info!("No Authorization header found.");
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}

pub async fn is_token_blacklisted(
    db_pool: &MySqlPool,
    token: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM token_blacklist 
        WHERE token = ? AND expires_at > ?
        "#,
        token,
        Utc::now()
    )
    .fetch_one(db_pool)
    .await?;

    Ok(result.unwrap_or(0) > 0)
}
