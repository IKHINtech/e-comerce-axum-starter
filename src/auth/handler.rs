use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use serde::Deserialize;

use sqlx::PgPool;

use crate::shared::utils::{create_jwt, hash_password, verify_password};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let hased_password = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };

    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, role) VALUES ($1, $2, $3, 'consumer')",
        payload.username,
        payload.email,
        hased_password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_user) => (StatusCode::CREATED, "Berhasil").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query!(
        "SELECT id, password_hash, role as \"role: String\" FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);
    if let Some(user) = user {
        if verify_password(&payload.password, &user.password_hash) {
            let role_str = user.role;
            let token = create_jwt(user.id.to_string(), role_str).unwrap();
            return (StatusCode::OK, Json(serde_json::json!({"token": token}))).into_response();
        }
    }
    (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
}
