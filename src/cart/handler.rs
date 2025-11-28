use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{cart::model::AddToChartRequest, shared::auth_guard::JWTAuth};

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
