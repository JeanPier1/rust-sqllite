mod models;
mod repository;
mod handlers;
mod routes;

use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::sync::Arc;
use dotenvy::dotenv;
use crate::repository::usuario_repository::UsuarioServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let usuario_service = Arc::new(UsuarioServiceImpl { pool });

    let app = routes::routes()
        .with_state(usuario_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 API corriendo en http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
