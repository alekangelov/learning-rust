use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, MatchedPath},
    http::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json as AJson, RequestPartsExt,
};

use serde_json::{json, Value};

// Make our own error that wraps `anyhow::Error`.
pub struct AppError {
    pub message: String,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AJson(json!({
              "status": "ERROR",
              "success": false,
              "message": format!("{}", self.message),
            })),
        )
            .into_response()
    }
}

// We define our own `Json` extractor that customizes the error from `axum::Json`
pub struct Json<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
where
    AJson<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, AJson<Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match AJson::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let payload = json!({
                    "message": rejection.body_text(),
                    "origin": "custom_extractor",
                    "path": path,
                });

                Err((rejection.status(), AJson(payload)))
            }
        }
    }
}
