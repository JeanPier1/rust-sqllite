use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: String,
    pub nombre: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NuevoUsuario {
    pub nombre: String,
    pub email: String,
}
