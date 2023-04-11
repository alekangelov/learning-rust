use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};

use super::error::AppError;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

struct Claims {
    sub: i32,
    exp: i64,
}
