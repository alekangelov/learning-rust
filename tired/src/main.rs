use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::{
    extract::State, http::StatusCode, response::Html, response::Json, routing::get, Router,
};

use chrono::offset::Utc;
use chrono::{naive::NaiveDate, DateTime};

use fake::locales::EN;
use fake::{
    faker::chrono::raw::DateTimeBefore,
    faker::phone_number::en::PhoneNumber,
    faker::{
        internet::en::FreeEmail,
        name::en::{FirstName, LastName},
    },
    Fake,
};
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct AppState {
    users: Mutex<Vec<User>>,
}

// Make our own error that wraps `anyhow::Error`.
struct AppError {
    message: String,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
              "message": format!("Something went wrong: {}", self.message),
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
struct JsonResponse<X> {
    result: X,
    success: bool,
}

impl AppState {
    fn get_users(&self) -> Vec<User> {
        let users = self.users.lock();
        match users {
            Ok(users) => users.clone(),
            Err(e) => {
                println!("Error: {:?}", e);
                Vec::new()
            }
        }
    }
}

#[derive(Serialize)]
struct RootResponse {
    name: &'static str,
    version: &'static str,
}

#[tokio::main]
async fn main() {
    let mut user_set = Vec::new();

    generate_users(&mut user_set);

    let state = AppState {
        users: Mutex::new(user_set),
    };

    let app_state = Arc::new(state);
    println!("Users: {:?}", app_state.get_users());
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/json", get(jason_handler))
        .route("/users", get(users_handler))
        .route("/users/:id", get(user_handler))
        .with_state(app_state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn jason_handler() -> (StatusCode, Json<RootResponse>) {
    (
        StatusCode::OK,
        Json(RootResponse {
            name: "axum",
            version: "0.2.0",
        }),
    )
}
#[derive(Debug, Serialize, Clone)]
struct User {
    id: u32,
    first_name: String,
    last_name: String,
    email: Option<String>,
    phone: Option<String>,
    date_of_birth: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            first_name: String::from(""),
            last_name: String::from(""),
            email: None,
            phone: None,
            date_of_birth: None,
        }
    }
}

fn generate_users(users: &mut Vec<User>) -> () {
    let start_dt_utc = DateTime::<Utc>::from_utc(
        NaiveDate::from_ymd_opt(1999, 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 59)
            .unwrap(),
        Utc,
    );
    for i in 0..1000 {
        users.push(User {
            id: i,
            email: FreeEmail().fake(),
            phone: PhoneNumber().fake(),
            date_of_birth: DateTimeBefore(EN, start_dt_utc).fake(),
            first_name: FirstName().fake(),
            last_name: LastName().fake(),
        })
    }
}

async fn users_handler(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<JsonResponse<Vec<User>>>) {
    (
        StatusCode::OK,
        Json(JsonResponse {
            result: state.get_users(),
            success: true,
        }),
    )
}

#[axum_macros::debug_handler]
async fn user_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<u32>,
) -> Result<Json<JsonResponse<User>>, AppError> {
    let users = state.get_users();
    let user = users.iter().find(|user| user.id == user_id);
    match user {
        Some(user) => Ok(Json(JsonResponse {
            result: user.clone(),
            success: true,
        })),
        None => Err(AppError {
            message: format!("User with id {} not found", user_id),
        }),
    }
}
