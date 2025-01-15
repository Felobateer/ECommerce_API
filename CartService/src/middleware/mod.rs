use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use futures::future::{BoxFuture, FutureExt};
use sqlx::MySqlPool;
use chrono::Utc;
use tracing::info;

pub struct LoggerMiddleware;

impl LoggerMiddleware {
    pub async fn handle<B>(
        req: Request<B>,
        next: Next<B>,
    ) -> Response {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        // Log the request details
        info!("Incoming request: {} {}", method, path);

        // Proceed to the next middleware or handler
        let response = next.run(req).await;

        // Log the response status
        let status = response.status();
        info!("Response status: {}", status);

        response
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
