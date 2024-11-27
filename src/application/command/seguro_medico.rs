use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use serde_json::Value;

pub async fn post_seguro_medico(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let tipo_seguro = payload
        .get("tipo_seguro")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let fecha_inicio = payload
        .get("fecha_inicio")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap();
    let fecha_final = payload
        .get("fecha_final")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap();
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let celular_contacto = payload
        .get("celular_contacto")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if let Err(e) = sqlx::query("call public.crear_seguro_medico($1, $2, $3, $4, $5, $6);")
        .bind(nombre)
        .bind(tipo_seguro)
        .bind(fecha_inicio)
        .bind(fecha_final)
        .bind(estado)
        .bind(celular_contacto)
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

pub async fn delete_seguro_medico(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_seguro_medico($1)")
        .bind(id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar seguro_medico: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn put_seguro_medico(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let tipo_seguro = payload
        .get("tipo_seguro")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let fecha_inicio = payload
        .get("fecha_inicio")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap();
    let fecha_final = payload
        .get("fecha_final")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap();
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let celular_contacto = payload
        .get("celular_contacto")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if let Err(e) =
        sqlx::query("call public.modificar_seguro_medico($1, $2, $3, $4, $5, $6, $7)")
            .bind(id)
            .bind(nombre)
            .bind(tipo_seguro)
            .bind(fecha_inicio)
            .bind(fecha_final)
            .bind(estado)
            .bind(celular_contacto)
            .execute(&state.get_db_pg())
            .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar paciente: {e}"),
        ));
    }
    Ok(StatusCode::OK)
}
