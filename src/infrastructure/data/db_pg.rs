use mongodb::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Result};
use crate::infrastructure::data::db_mongo::ClientState;
use crate::infrastructure::data::get_db::GetDb;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
    
}
impl GetDb<PgPool> for AppState {
    fn get_db(&self) -> &PgPool {
        &self.db
    }
}
pub async fn connect_db() -> Result<AppState, sqlx::Error> {
    let database_url = "postgres://postgres:root@localhost:5432/doctorya";

    let db = match PgPoolOptions::new()
        .max_connections(5)
        .min_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error al conectar a la base de datos: {:?}", e);
            return Err(e);
        }
    };

    match sqlx::query("SET search_path TO public")
        .execute(&db)
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error al ejecutar la consulta SET search_path: {:?}", e);
            return Err(e);
        }
    }

    let state = AppState::new(db);

    Ok(state)
}

