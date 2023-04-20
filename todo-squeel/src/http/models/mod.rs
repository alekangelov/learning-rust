use serde::Serialize;
use sqlx::{FromRow, PgPool};

use super::error::AppError;

#[derive(FromRow, Debug, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub const TABLE: &'static str = "users";
}

pub async fn find_user_by_id(id: &str, pool: PgPool) -> Result<User, AppError> {
    sqlx::query_as::<_, User>("select * from users where id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError {
            message: "Could not find user!".to_string(),
            code: None,
        })
}

#[derive(FromRow, Debug, Serialize, Clone)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub owner: User,
}

impl Todo {
    pub const TABLE: &'static str = "todos";
}

#[derive(FromRow, Debug, Serialize, Clone)]
pub struct Profile {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

impl Profile {
    pub const TABLE: &'static str = "profiles";
}
