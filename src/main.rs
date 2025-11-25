mod auth;
mod router;
mod shared;
mod users;

use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Setup Logging (Tracing) agar output console rapi
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Setup Database (Contoh koneksi ke Postgres)
    // Pastikan ada file .env berisi DATABASE_URL="postgres://user:pass@localhost/db_name"
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or("postgres://...".to_string());

    // Note: Karena mungkin Anda belum punya DB jalan, kita pakai mock connection dulu
    // Di production: gunakan PgPoolOptions::new().connect(&db_url).await.unwrap();
    let pool = PgPoolOptions::new().connect_lazy(db_url.as_str()).unwrap();

    // 3. Buat App Router dari modul router.rs
    let app = router::create_router(pool);

    // 4. Tentukan Port dan Jalankan Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server berjalan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
