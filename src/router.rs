use crate::{
    auth::handler::{login_handler, register_handler},
    products::handler::{create_product_handler, delete_product_handler, get_products_handler},
    users::handler::{create_user_handler, get_user_handler},
};
use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        // publik
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        // --- PRODUCT ROUTES ---
        .route("/products", get(get_products_handler)) // GET: Publik
        .route("/products", post(create_product_handler)) // POST: Butuh Token Admin
        .route("/products/:id", delete(delete_product_handler)) // DELETE: Butuh Token Admin
        .route("/users", post(create_user_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/health", get(|| async { "Server is running! 🚀" }))
        // Menyuntikkan Database Pool ke semua route di atas
        .with_state(pool)
}
