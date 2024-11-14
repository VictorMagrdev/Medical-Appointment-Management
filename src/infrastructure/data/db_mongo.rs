use mongodb::{Client, options::{ClientOptions}};
use std::error::Error;
use crate::infrastructure::data::get_db::GetDb;

#[derive(Clone)]
pub struct ClientState {
    db: Client,
}

impl ClientState {
    fn new(db: Client) -> Self {
        Self { db }
    }
}

impl GetDb<Client> for ClientState {
    fn get_db(&self) -> &Client {
        &self.db
    }
}
pub async fn connect_db_mongo() -> Result<ClientState, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let state = ClientState::new(client);

    Ok(state)
}