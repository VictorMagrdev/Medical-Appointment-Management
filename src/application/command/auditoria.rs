use crate::infrastructure::data::db_mongo::ClientState;
use crate::infrastructure::data::get_db::GetDb;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn post_auditoria(
    State(client): State<ClientState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    Ok(StatusCode::CREATED)
}

pub async fn delete_auditoria(
    State(client): State<ClientState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    Ok(StatusCode::OK)
}

pub async fn put_auditoria(
    State(client): State<ClientState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    Ok(StatusCode::OK)
}
