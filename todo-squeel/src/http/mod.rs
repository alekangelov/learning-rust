use std::sync::Arc;

use axum::{routing::get, Extension, Json, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod auth;
mod error;
mod jwt;
mod models;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[derive(Clone)]
pub struct FakeState {}

pub async fn serve(pg_pool: PgPool) {
    let app_state = Arc::new(AppState { db: pg_pool });
    let api_router = Router::new().nest("/auth", auth::router());
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .into_inner();

    let f_state = FakeState {};

    let app = Router::new()
        .route("/", get(index))
        .nest("/api", api_router)
        .layer(middleware_stack)
        .layer(Extension(app_state))
        .with_state(f_state);

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
