use crate::{
    products::model::{CreateProductRequest, Product},
    shared::auth_guard::JWTAuth,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

pub async fn get_products_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    let products = sqlx::query_as!(
        Product,
        "SELECT id, name, description, price, stock FROM products ORDER BY id DESC"
    )
    .fetch_all(&pool)
    .await;
    match products {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response(),
    }
}

pub async fn create_product_handler(
    State(pool): State<PgPool>,
    claims: JWTAuth, // <--- Middleware cek token di sini
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    // 1. Cek Role Admin
    if claims.role != "admin" {
        return (StatusCode::FORBIDDEN, "Akses ditolak: Khusus Admin").into_response();
    }

    // 2. Insert ke Database
    let result = sqlx::query!(
        "INSERT INTO products (name, description, price, stock) VALUES ($1, $2, $3, $4) RETURNING id",
        payload.name,
        payload.description,
        payload.price,
        payload.stock
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(rec) => (
            StatusCode::CREATED,
            Json(serde_json::json!({"message": "Produk berhasil dibuat", "id": rec.id})),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Gagal: {}", e)).into_response(),
    }
}

pub async fn delete_product_handler(
    State(pool): State<PgPool>,
    claims: JWTAuth,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    if claims.role != "admin" {
        return (StatusCode::FORBIDDEN, "Akses ditolak: Khusus Admin").into_response();
    }

    let result = sqlx::query!("DELETE FROM products WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, "Produk dihapus").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Gagal menghapus").into_response(),
    }
}
