use crate::infrastructure::data::db_mongo::ClientState;
use axum::extract::{Path, State};
use axum::http::{Error, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn post_auditoria(
    State(client): State<ClientState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, Error> {
    Ok(StatusCode::CREATED)
}
 
 pub async fn delete_auditoria(
     State(client): State<ClientState>,
     Path(id): Path<i32>,
 ) -> Result<impl IntoResponse, Error>  {
     todo!("a futuro");
     Ok(StatusCode::OK)
 }
 
 pub async fn put_auditoria(
     State(client): State<ClientState>,
     Path(id): Path<i32>,
     Json(payload): Json<Value>,
 ) -> Result<impl IntoResponse, Error>  {
     todo!("a futuro");
     Ok(StatusCode::OK)
     
 }
