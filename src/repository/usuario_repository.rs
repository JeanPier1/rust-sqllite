use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::models::usuario::{Usuario, NuevoUsuario};
use uuid::Uuid;

#[async_trait]
pub trait UsuarioService {
    async fn get_all(&self) -> Result<Vec<Usuario>, sqlx::Error>;
    async fn create(&self, datos: NuevoUsuario) -> Result<(), sqlx::Error>;
    async fn get_byid(&self, id: String) -> Result<Usuario, sqlx::Error>;
    async fn update(&self, id: String, datos: NuevoUsuario) -> Result<(), sqlx::Error>;
    async fn delete(&self, id: String) -> Result<(), sqlx::Error>;
}

pub struct UsuarioServiceImpl {
    pub pool: SqlitePool,
}

#[async_trait]
impl UsuarioService for UsuarioServiceImpl {
    async fn get_all(&self) -> Result<Vec<Usuario>, sqlx::Error> {
        sqlx::query_as!(Usuario, "SELECT id, nombre, email FROM usuarios")
            .fetch_all(&self.pool)
            .await
    }

    async fn get_byid(&self, id: String) -> Result<Usuario, sqlx::Error> {
        sqlx::query_as!(Usuario, "SELECT * FROM usuarios WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await
    }

    async fn create(&self, datos: NuevoUsuario) -> Result<(), sqlx::Error> {
        let nuevo_uuid = Uuid::new_v4().to_string();
        sqlx::query!("INSERT INTO usuarios (id, nombre, email) VALUES (?,?, ?)", nuevo_uuid,datos.nombre, datos.email)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update(&self, id: String, datos: NuevoUsuario) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE usuarios SET nombre = ?, email = ? WHERE id = ?", datos.nombre, datos.email, id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, id:String) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM usuarios WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
