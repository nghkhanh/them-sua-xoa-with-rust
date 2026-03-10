mod config;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use clorinde::deadpool_postgres::Pool;
use clorinde::queries::users::{
    create_user, delete_user, get_users, update_user,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    pool: Arc<Pool>,
}

struct AppError(String);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

impl<E: std::fmt::Display> From<E> for AppError {
    fn from(err: E) -> Self {
        AppError(err.to_string())
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let pool = db::create_pool(&config.database_url);
    let shared_state = AppState {
        pool: Arc::new(pool),
    };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/users", get(list_users))
        .route("/users", post(add_user))
        .route("/users/{id}", put(edit_user))
        .route("/users/{id}", delete(remove_user))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    "Hello from the server!"
}

#[derive(serde::Serialize)]
struct UserResponse {
    id: i32,
    email: String,
}

#[axum::debug_handler]
async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<UserResponse>>, AppError> {
    let client = state.pool.get().await?;
    
    let users = get_users()
        .bind(&client)
        .all()
        .await?;

    let response = users.into_iter().map(|u| UserResponse {
        id: u.id,
        email: u.email
    }).collect();

    Ok(Json(response))
}

#[derive(Deserialize)]
struct CreateUserPayload {
    email: String,
    external_id: String,
}

#[axum::debug_handler]
async fn add_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<StatusCode, AppError> {
    let client = state.pool.get().await?;
    
    create_user()
        .bind(&client, &payload.email, &payload.external_id)
        .await?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct UpdateUserPayload {
    email: String,
}

#[axum::debug_handler]
async fn edit_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<StatusCode, AppError> {
    let client = state.pool.get().await?;
    
    update_user()
        .bind(&client, &payload.email, &id)
        .await?;

    Ok(StatusCode::OK)
}

#[axum::debug_handler]
async fn remove_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let client = state.pool.get().await?;
    
    delete_user()
        .bind(&client, &id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
