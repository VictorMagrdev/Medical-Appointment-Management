use crate::infrastructure::data::db_pg::AppState;
use crate::infrastructure::data::get_db::GetDb;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use serde_json::Value;

pub async fn post_paciente(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload.get("nombre").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let identificacion = payload.get("identificacion").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let fecha_nacimiento = payload.get("fecha_nacimiento")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).unwrap();
    let sexo = payload.get("sexo").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let direccion = payload.get("direccion").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let email = payload.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let celular = payload.get("celular").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let seguro_id = payload.get("seguro_id").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    if let Err(e) = sqlx::query(
        "call public.crear_paciente($1, $2, $3, $4::sexo, $5, $6, $7, $8);",
    )
        .bind(nombre)
        .bind(identificacion)
        .bind(fecha_nacimiento)
        .bind(sexo)
        .bind(direccion)
        .bind(email)
        .bind(celular)
        .bind(seguro_id)
        .execute(state.get_db())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear paciente: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn delete_paciente(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_paciente($1)")
        .bind(id)
        .execute(state.get_db())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar paciente: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn put_paciente(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload.get("nombre").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let identificacion = payload.get("identificacion").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let fecha_nacimiento = payload.get("fecha_nacimiento")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).unwrap();
    let sexo = payload.get("sexo").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let direccion = payload.get("direccion").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let email = payload.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let celular = payload.get("celular").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let seguro_id = payload.get("seguro_id").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    if let Err(e) =
        sqlx::query("SELECT * FROM public.modificar_paciente($1, $2, $3, $4::sexo, $5, $6, $7, $8, $9)")
            .bind(id)
            .bind(nombre)
            .bind(identificacion)
            .bind(fecha_nacimiento)
            .bind(sexo)
            .bind(direccion)
            .bind(email)
            .bind(celular)
            .bind(seguro_id)
            .fetch_one(state.get_db())
            .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar paciente: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
