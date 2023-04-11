use argon2::{password_hash::SaltString, PasswordHash, Argon2};
use chrono::{Duration, Utc};
use axum::{
    extract::{ State},
    routing::post,
    Json, Router,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use validator::{Validate };

use super::AppState;
use super::{error::AppError, user::User};

pub fn router() -> Router {
    Router::new()
        .route("/api/login", post(login))
        .route("/api/register", post(register))
}

#[derive(Debug, Validate, Deserialize)]
struct LoginBody {
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 6))]
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterBody {
    #[validate(length(min = 1))]
    username: String,

    #[validate(length(min = 6))]
    password: String,

}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: i64,
}

struct AuthResponse {
    token: String,
}

async fn login(
    Json(body): Json<LoginBody>,
    State(state): State<AppState>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_one(&state.db)
        .await {
            Ok(user) => user,
            Err(_) => {
                return Err(AppError {
                    message: "User not found".to_string(),
                })
            }
        };
    let jwt = generate_jwt(&user)?;
    Ok(Json(AuthResponse { token: jwt }))
}




fn generate_jwt(user: &User) -> Result<String, AppError> {
  let claims = Claims {
            sub: user.id,
            exp: (Utc::now() + Duration::days(1)).timestamp(),
        }
  match  encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref())
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(AppError{
            message: "Error generating token".to_string(),
        })
    }
}


async fn hash_password(password: String) -> Result<String, AppError> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<bool, AppError> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash"))
}
