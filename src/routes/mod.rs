use axum::{routing::{get,put}, Router};
use std::sync::Arc;
use crate::handlers::user_handler;
use crate::repository::usuario_repository::{UsuarioServiceImpl};


pub fn routes() -> Router<Arc<UsuarioServiceImpl>> {
    Router::new()
        .route("/usuarios", get(user_handler::get_all).post(user_handler::save))
        .route("/usuarios/:id", put(user_handler::update).delete(user_handler::delete).get(user_handler::get_byid))
}
