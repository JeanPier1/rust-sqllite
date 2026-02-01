use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    response::Response,
};
use tower::ServiceExt;
use serde_json::{json, Value};
use std::sync::Arc;
use http_body_util::BodyExt;

use first::routes::routes;
use first::repository::usuario_repository::UsuarioServiceImpl;

async fn app_factory() -> Router {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let service = Arc::new(UsuarioServiceImpl { pool });
    routes().with_state(service)
}


#[tokio::test]
async fn test_create_user() {
    let app = app_factory().await;
    let res = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/usuarios")
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"nombre": "Jean", "email": "jean@test.com"}).to_string())).unwrap()
    ).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_user() {
    let app = app_factory().await;
    let user_id = crear_usuario_auxiliar(&app).await;

    let res = app.oneshot(
        Request::builder()
            .method("PUT")
            .uri(format!("/usuarios/{}", user_id))
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"nombre": "Edit", "email": "e@test.com"}).to_string())).unwrap()
    ).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}



#[tokio::test]
async fn test_delete_usuario() {
    let app = app_factory().await;
    let user_id = crear_usuario_auxiliar(&app).await;

    let res = app.clone().oneshot(
        Request::builder()
            .method("DELETE")
            .uri(format!("/usuarios/{}", user_id))
            .body(Body::empty())
            .unwrap()
    ).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    println!("✅ Test DELETE: Usuario {} eliminado correctamente", user_id);
}

#[tokio::test]
async fn test_update_usuario() {
    let app = app_factory().await;
    let user_id = crear_usuario_auxiliar(&app).await;

    let res = app.clone().oneshot(
        Request::builder()
            .method("PUT")
            .uri(format!("/usuarios/{}", user_id))
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"nombre": "Nuevo", "email": "n@t.com"}).to_string()))
            .unwrap()
    ).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    println!("✅ Test UPDATE: Usuario {} actualizado", user_id);
}

async fn crear_usuario_auxiliar(app: &Router) -> String {
    let _ = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/usuarios")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"nombre": "Temp", "email": "temp@t.com"}).to_string()))
                .unwrap()
        )
        .await
        .unwrap();

    let res: Response = app.clone()
        .oneshot(
            Request::builder().uri("/usuarios").body(Body::empty()).unwrap()
        )
        .await
        .unwrap();

    let body = res
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let usuarios: Value = serde_json::from_slice(&body).expect("Error al parsear JSON");

    usuarios[0]["id"]
        .as_str()
        .expect("El campo ID no existe o no es un String")
        .to_string()
}
