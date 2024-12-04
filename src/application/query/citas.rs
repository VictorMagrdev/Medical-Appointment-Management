use axum::body::to_bytes;
use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use sqlx::Row;

pub async fn obtener_dia_cita(
    State(state): State<AppState>,
    Path(cita_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_dia_cita($1) as dia")
        .bind(cita_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el día de la cita: {e}"),
            ));
        }
    };

    let dia: Option<chrono::NaiveDate> = row.get("dia");

    Ok(Json(json!({ "dia": dia.unwrap_or_default().to_string() })))
}

pub async fn obtener_hora_cita(
    State(state): State<AppState>,
    Path(cita_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_hora_cita($1) as hora")
        .bind(cita_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener la hora de la cita: {e}"),
            ));
        }
    };

    let hora: Option<chrono::NaiveTime> = row.get("hora");

    Ok(Json(
        json!({ "hora": hora.unwrap_or_default().to_string() }),
    ))
}

pub async fn obtener_medico_cita(
    State(state): State<AppState>,
    Path(cita_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_medico_cita($1) as medico")
        .bind(cita_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el médico de la cita: {e}"),
            ));
        }
    };

    let medico: Option<String> = row.get("medico");

    Ok(Json(json!({ "medico": medico.unwrap_or_default() })))
}

//Para probar
pub async fn obtener_estado_cita(
    State(state): State<AppState>,
    Path(cita_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_estado_cita($1) as estado")
        .bind(cita_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el estado de la cita: {e}"),
            ));
        }
    };

    let estado: Option<String> = row.get("estado");

    Ok(Json(json!({ "estado": estado.unwrap_or_default() })))
}

pub async fn verificar_estado_cita(
    State(state): State<AppState>,
    Path(cita_id): Path<i32>,
) -> Result<bool, (StatusCode, String)> {
    let response = obtener_estado_cita(State(state), Path(cita_id)).await;
    let body = axum::body::to_bytes(response.into_response().into_body(), usize::MAX).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error al leer el cuerpo de la respuesta".to_string()))?;

    let payload: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error al deserializar la respuesta JSON".to_string()))?;

    let estado_completada = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .map_or(false, |estado| estado == "completada");

    Ok(estado_completada)
}
