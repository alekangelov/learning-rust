use axum::Router;

use self::{auth::auth_router, user::user_router};

mod auth;
mod error;
mod helpers;
mod jwt;
mod user;

pub fn api_router() -> Router {
    Router::new()
        .nest("/api/auth", auth_router())
        .nest("/api/user", user_router())
}
