use axum::http::{StatusCode};
use serde_json::json;

pub (crate) fn endpoint_error (e: sqlx::Error) -> (StatusCode, String) {(
    StatusCode:: INTERNAL_SERVER_ERROR,
    json!({ "success": false, "message": e.to_string() }).to_string(),
)}