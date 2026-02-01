## Rust + Axum

Este proyecto es una aplicación de ejemplo para el curso de Rust + Axum.

### Requisitos

- Rust
- Cargo
- SQLite
- SQLx
- dotenvy

async-trait

### Instalación

1. Clonar el repositorio
2. Ejecutar `cargo run`

### Ejecución

Para ejecutar el proyecto, se debe crear un archivo `.env` en la raíz del proyecto con las siguientes variables:

```dotenv
DATABASE_URL=sqlite:///data.db
```

Luego, se puede ejecutar el proyecto con `cargo run`.
