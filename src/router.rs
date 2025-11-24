use crate::users::handler::{create_user_handler, get_user_handler};
use axum::{Router, routing::get, routing::post};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(create_user_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/health", get(|| async { "Server is running! 🚀" }))
        // Menyuntikkan Database Pool ke semua route di atas
        .with_state(pool)
}
