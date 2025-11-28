use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::{
    cart::model::{AddToChartRequest, CartItemDTO},
    shared::auth_guard::JWTAuth,
};

pub async fn add_to_chat_handler(
    State(pool): State<sqlx::PgPool>,
    claims: JWTAuth,
    Json(payload): Json<AddToChartRequest>,
) -> impl IntoResponse {
    if payload.quantity <= 0 {
        return (StatusCode::BAD_REQUEST, "Quantity must be greater than 0").into_response();
    }
    let result = sqlx::query!(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, product_id) 
        DO UPDATE SET quantity = cart_items.quantity + $3
        "#,
        claims.user_id,
        payload.product_id,
        payload.quantity
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "Barang masuk keranjang").into_response(),
        Err(e) => {
            // Error handling jika product_id tidak ada di tabel products
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Gagal: {}", e)).into_response()
        }
    }
}

pub async fn get_cart_handler(State(pool): State<PgPool>, claims: JWTAuth) -> impl IntoResponse {
    // QUERY JOIN
    // Mengambil data cart + nama produk + hitung total harga
    let items = sqlx::query_as!(
        CartItemDTO,
        r#"
        SELECT 
            c.id as cart_id,
            p.id as product_id,
            p.name as product_name,
            p.price,
            c.quantity,
            (p.price * c.quantity) as "total_price!" -- Tanda ! memberitahu sqlx ini pasti tidak null
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        ORDER BY c.id ASC
        "#,
        claims.user_id
    )
    .fetch_all(&pool)
    .await;

    match items {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response(),
    }
}
