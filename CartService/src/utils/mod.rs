use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Validates a given string as a valid UUID
pub fn validate_uuid(id: &str) -> Result<Uuid, String> {
    match Uuid::parse_str(id) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err("Invalid UUID format".to_string()),
    }
}

/// Generates the current UTC timestamp
pub fn current_utc_time() -> DateTime<Utc> {
    Utc::now()
}

/// Creates a generic JSON error response
pub fn json_error_response(status_code: StatusCode, message: &str) -> impl IntoResponse {
    let error_body = json!({
        "error": message
    });

    (status_code, Json(error_body))
}

/// Wraps a successful JSON response
pub fn json_success_response<T>(status_code: StatusCode, data: T) -> impl IntoResponse
where
    T: serde::Serialize,
{
    (status_code, Json(json!(data)))
}
