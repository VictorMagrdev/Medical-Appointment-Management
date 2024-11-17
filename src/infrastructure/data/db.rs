use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Result};
use mongodb::{options::ClientOptions, Client};

#[derive(Clone)]
pub struct AppState {
    db_pg: PgPool,
    db_mongo: Client,
}

impl AppState {
    fn new(db_pg: PgPool,db_mongo: Client) -> Self {
        Self { db_pg, db_mongo }
    }
    pub fn get_db_pg(&self) -> PgPool { self.db_pg.clone() }
    pub fn get_db_mongo(&self) -> Client { self.db_mongo.clone() }
}
async fn connect_pg_db() -> Result<PgPool> {
    let database_url = "postgres://postgres:postgres@localhost:5432/doctorya";

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

    match sqlx::query("SET search_path TO public").execute(&db).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error al ejecutar la consulta SET search_path: {:?}", e);
            return Err(e);
        }
    }
    Ok(db)
}

pub async fn connect_db_mongo() -> Result<Client> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    
    Ok(client)
}
pub async fn connect_db() -> Result<AppState, sqlx::Error> {
    let db_pg = connect_pg_db().await?;
    let db_client = connect_db_mongo().await?;
    let state = AppState::new(db_pg,db_client);

    Ok(state)
}
