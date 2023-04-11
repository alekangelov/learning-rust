use axum::{
    extract::{Extension, Path, State},
    prelude::*,
    routing::post,
    Json, Router,
};

use super::user::User;
use super::AppState;

pub fn router() -> Router {
    Router::new()
        .route("/api/login", post(login))
        .route("/api/register", post(register))
        .route("/api/refreshToken", post(refreshToken))
}

struct LoginBody {
    username: String,
    password: String,
}

struct LoginResponse {
    token: String,
    refreshToken: String,
}

async fn login(
    Json(body): Json<LoginBody>,
    State(state): State<AppState>,
) -> Result<Json<LoginResponse>, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_one(&state.db)
        .await?;

    if !user.verify_password(&body.password) {
        return Err(Error::Unauthorized);
    }

    let token = user.generate_token()?;
    let refreshToken = user.generate_refresh_token()?;

    Ok(Json(LoginResponse {
        token,
        refreshToken,
    }))
}
