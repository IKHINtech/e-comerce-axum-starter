use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
    email: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    email: String,
}

pub async fn get_user_handler(
    Path(id): Path<u64>,
    State(_pool): State<PgPool>,
) -> impl IntoResponse {
    let user = User {
        id,
        username: "John".to_string(),
        email: "john@doe".to_string(),
    };

    Json(user)
}

pub async fn create_user_handler(
    State(_pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User {
        id: 1,
        username: req.username,
        email: req.email,
    };
    print!("{}", user.username);
    (axum::http::StatusCode::CREATED, "created")
}
