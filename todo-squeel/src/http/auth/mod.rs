use anyhow::Error;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{extract::State, routing::post, Json, Router};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::AppState;
use super::{error::AppError, user::User};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

#[derive(Debug, Validate, Serialize, Deserialize)]
struct LoginBody {
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

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    token: String,
}

#[axum_macros::debug_handler]
async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Err(AppError {
                message: "User not found".to_string(),
            })
        }
    };
    if !verify_password(body.password, user.password.clone()).await {
        return Err(AppError {
            message: "Invalid password".to_string(),
        });
    }
    let jwt = generate_jwt(&user)?;
    Ok(Json(AuthResponse { token: jwt }))
}

#[axum_macros::debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => {
            return Err(AppError {
                message: "User already exists".to_string(),
            })
        }
        Err(_) => {
            let password_hash = hash_password(body.password.clone()).await?;
            let user_model = User {
                id: 0,
                username: body.username.clone(),
                password: password_hash,
            };
            let user = sqlx::query_as::<_, User>(
                "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *",
            )
            .bind(&user_model.username)
            .bind(&user_model.password)
            .fetch_one(&state.db)
            .await;
            user
        }
    };
    match user {
        Ok(user) => {
            let jwt = generate_jwt(&user)?;
            Ok(Json(AuthResponse { token: jwt }))
        }
        Err(_) => Err(AppError {
            message: "Error creating user".to_string(),
        }),
    }
}

fn generate_jwt(user: &User) -> Result<String, AppError> {
    let claims = Claims {
        sub: user.id,
        exp: (Utc::now() + Duration::days(1)).timestamp(),
    };
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(AppError {
            message: "Error generating token".to_string(),
        }),
    }
}

async fn hash_password(password: String) -> Result<String, AppError> {
    let hash = tokio::task::spawn_blocking(move || -> Result<String, Error> {
        let salt = SaltString::generate(rand::thread_rng());
        let hash = PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e));
        match hash {
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(anyhow::anyhow!("Error generating hash")),
        }
    })
    .await;
    match hash {
        Ok(hash) => match hash {
            Ok(hash) => Ok(hash),
            Err(_) => Err(AppError {
                message: "Error generating hash".to_string(),
            }),
        },
        Err(_) => Err(AppError {
            message: "Error generating hash".to_string(),
        }),
    }
}

async fn verify_password(password: String, password_hash: String) -> bool {
    let verification = tokio::task::spawn_blocking(move || -> Result<(), Error> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => anyhow::anyhow!("invalid password"),
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await;
    match verification {
        Ok(verification) => match verification {
            Ok(_) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}
