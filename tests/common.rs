use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;
use first::repository::usuario_repository::UsuarioServiceImpl;
use first::routes;
use axum::Router;

pub struct TestApp {
    pub app: Router,
    pub pool: SqlitePool,
}

pub async fn spawn_app() -> TestApp {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("No se pudo conectar a la DB de prueba");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("No se pudieron ejecutar las migraciones");

    let service = Arc::new(UsuarioServiceImpl { pool: pool.clone() });
    let app = routes::routes().with_state(service);

    TestApp { app, pool }
}
