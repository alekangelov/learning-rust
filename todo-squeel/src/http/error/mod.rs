use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

// Make our own error that wraps `anyhow::Error`.
pub struct AppError {
    message: String,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
              "message": format!("Something went wrong: {}", self.message),
            })),
        )
            .into_response()
    }
}
