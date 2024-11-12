use sqlx::{PgPool, Result, PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
}

pub async fn connect_db() -> Result<AppState> {
    let database_url = "postgres://usuario:contraseña@localhost:5432/mi_base_de_datos";

    let db = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(5)
        .connect(database_url)
        .await?;

    let state = AppState::new(db);
    Ok(state)
}
