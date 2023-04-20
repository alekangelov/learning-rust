use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use super::{auth::JwtClaims, error::AppError, models::find_user_by_id};


const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";

pub async fn auth_middleware<S>(
    mut request: Request<S>,
    next: Next<S>,
) -> Result<Response, AppError> {
    let auth_header = request.headers().get(AUTHORIZATION).ok_or(AppError {
        message: "Missing authorization header".to_string(),
        code: Some(StatusCode::UNAUTHORIZED),
    })?;

    let token = auth_header
        .to_str()
        .map_err(|_| AppError {
            message: "Invalid authorization header".to_string(),
            code: Some(StatusCode::UNAUTHORIZED),
        })?
        .strip_prefix(BEARER)
        .ok_or(AppError {
            message: "Invalid authorization header".to_string(),
            code: Some(StatusCode::UNAUTHORIZED),
        })?;
    let decoded = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError {
        message: "Could not decode token".to_string(),
        code: None,
    })?;

    let user = find_user_by_id(&decoded.claims.sub, pool)

    let extensions = request.extensions_mut();
    extensions.insert(decoded);

    Ok(next.run(request).await)
}
