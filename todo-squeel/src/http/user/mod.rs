use axum::{
    extract::{Extension, Path},
    prelude::*,
    routing::get,
    Json, Router,
};

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};

pub fn router() {
    Router::new()
        .route("/users", get(index))
        .route("/users/:id", get(show))
        .route("/users", post(create))
}

#[derive(sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}

struct Claims {
    sub: i32,
    exp: i64,
}

impl User {
    pub fn verify_password(&self, password: String) -> bool {
        verify_password(self.password, password.to_string())
    }

    to_jwt(&:self) -> String {
        let claims = Claims {
            sub: self.id,
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jwt_secret()).unwrap();
        token
    }
}

async fn hash_password(password: String) -> Result<String> {
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

async fn verify_password(password: String, password_hash: String) -> Result<()> {
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
    .context("panic in verifying password hash")??)
}
