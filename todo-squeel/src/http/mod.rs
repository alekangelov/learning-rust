use axum::{routing::get, Json, Router};
use sqlx::PgPool;

mod auth;
mod user;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

pub async fn serve(pg_pool: PgPool) {
    let app_state = AppState { db: pg_pool };
    let app = Router::new().route("/", get(index)).with_state(app_state);
    app.merge(auth::router());
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on http://localhost:3000");
}

#[derive(serde::Serialize)]
struct Index {
    message: String,
}

async fn index() -> Json<Index> {
    Json(Index {
        message: "Hello, world!".to_string(),
    })
}
