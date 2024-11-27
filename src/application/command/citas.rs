use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{NaiveDate, NaiveTime};
use serde_json::Value;

pub async fn post_cita(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let fecha = payload
        .get("fecha")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .ok_or((
            StatusCode::BAD_REQUEST,
            "La fecha es obligatoria y debe estar en formato YYYY-MM-DD".to_string(),
        ))?;
    let hora = payload
        .get("hora")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveTime::parse_from_str(s, "%H:%M:%S").ok())
        .ok_or((
            StatusCode::BAD_REQUEST,
            "La hora es obligatoria y debe estar en formato HH:MM:SS".to_string(),
        ))?;
    let motivo = payload
        .get("motivo")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let paciente_id = payload.get("paciente_id").and_then(|v| v.as_i64()).ok_or((
        StatusCode::BAD_REQUEST,
        "El ID del paciente es obligatorio y debe ser un número entero".to_string(),
    ))? as i32;
    let medico_id = payload.get("medico_id").and_then(|v| v.as_i64()).ok_or((
        StatusCode::BAD_REQUEST,
        "El ID del médico es obligatorio y debe ser un número entero".to_string(),
    ))? as i32;

    if let Err(e) = sqlx::query("CALL public.crear_cita($1, $2, $3, $4, $5)")
        .bind(fecha)
        .bind(hora)
        .bind(motivo)
        .bind(paciente_id)
        .bind(medico_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear la cita: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn put_estado_cita(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .ok_or((
            StatusCode::BAD_REQUEST,
            "El estado de la cita es obligatorio".to_string(),
        ))?
        .to_string();

    if let Err(e) = sqlx::query("CALL public.cambiar_estado_cita($1, $2)")
        .bind(id)
        .bind(estado)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al cambiar el estado de la cita: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
