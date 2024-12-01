use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use serde_json::Value;

pub async fn post_examen(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let costo: f32 = payload.get("costo").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let cubre_seguro = payload
        .get("cubre_seguro")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fecha_realizacion = payload
        .get("fecha_realizacion")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap();
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let historia_clinica_id = payload
        .get("historia_clinica_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("call public.crear_examen($1, $2::decimal, $3, $4, $5, $6);")
        .bind(&nombre)
        .bind(costo)
        .bind(cubre_seguro)
        .bind(fecha_realizacion)
        .bind(estado)
        .bind(historia_clinica_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear paciente: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn delete_examen(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_examen($1)")
        .bind(id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar seguro_examen: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn put_examen(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let costo = payload.get("costo").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let cubre_seguro = payload
        .get("cubre_seguro")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fecha_realizacion = payload
        .get("fecha_realizacion")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap();
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let historia_clinica_id = payload
        .get("historia_clinica_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("call public.modificar_examen($1, $2, $3::decimal, $4, $5, $6, $7)")
        .bind(id)
        .bind(nombre)
        .bind(costo)
        .bind(cubre_seguro)
        .bind(fecha_realizacion)
        .bind(estado)
        .bind(historia_clinica_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar examen: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
