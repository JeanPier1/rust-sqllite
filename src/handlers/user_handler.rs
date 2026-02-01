use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::repository::usuario_repository::{UsuarioService, UsuarioServiceImpl};
use crate::models::usuario::NuevoUsuario;

pub async fn get_all(State(svc): State<Arc<UsuarioServiceImpl>>) -> Json<Value> {
    match svc.get_all().await {
        Ok(u) => Json(json!(u)),
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}

pub async fn get_byid(State(svc): State<Arc<UsuarioServiceImpl>>, Path(id): Path<String>) -> Json<Value> {
    match svc.get_byid(id).await {
        Ok(u) => Json(json!(u)),
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}

pub async fn save(State(svc): State<Arc<UsuarioServiceImpl>>, Json(payload): Json<NuevoUsuario>) -> Json<Value> {
    match svc.create(payload).await {
        Ok(_) => Json(json!({"status": "creado"})),
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}

pub async fn update(State(svc): State<Arc<UsuarioServiceImpl>>, Path(id): Path<String>, Json(payload): Json<NuevoUsuario>) -> Json<Value> {
    match svc.update(id, payload).await {
        Ok(_) => Json(json!({"status": "actualizado"})),
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}

pub async fn delete(State(svc): State<Arc<UsuarioServiceImpl>>, Path(id): Path<String>) -> Json<Value> {
    match svc.delete(id).await {
        Ok(_) => Json(json!({"status": "eliminado"})),
        Err(e) => Json(json!({"error": e.to_string()})),
    }
}
