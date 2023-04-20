use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    BoxError, Json as AJson,
};

use serde_json::json;

// Make our own error that wraps `anyhow::Error`.
pub struct AppError {
    pub message: String,
    pub code: Option<StatusCode>,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            AJson(json!({
              "status": "ERROR",
              "success": false,
              "message": format!("{}", self.message),
            })),
        )
            .into_response()
    }
}
// handle errors by converting them into something that implements
// `IntoResponse`
pub async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}

// pub async fn handle_any_error(err: BoxError) -> (StatusCode, String) {
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         format!("Unhandled internal error: {}", err),
//     )
// }

pub fn handle_any_error(err: BoxError) -> Response {
    let message = format!("{}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        AJson(json!({
          "status": "ERROR",
          "success": false,
          "message": message,
        })),
    )
        .into_response()
}
