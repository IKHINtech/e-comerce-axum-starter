use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct AddToChartRequest {
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct CartItemDTO {
    pub cart_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub price: i64,
    pub quantity: i64,
    pub total_price: i64,
}
