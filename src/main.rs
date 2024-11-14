use crate::api::router::create_router;
use crate::infrastructure::data::db_pg::{connect_db, AppState};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::routing::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use crate::infrastructure::data::db_mongo::{connect_db_mongo, ClientState};

mod api;
mod application;
mod infrastructure;
mod r#macro;

#[tokio::main]
async fn main() {
    let state: AppState = connect_db().await.unwrap();
    let client: ClientState = connect_db_mongo().await.unwrap();
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_headers(vec![AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let routes: Router = create_router(state,client).layer(cors);
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server started successfully");
    axum::serve(listener, routes).await.unwrap();
}
