
use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn post_medico(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let identificacion = payload
        .get("identificacion")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let registro_medico = payload
        .get("registro_medico")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let especialidad_id = payload
        .get("especialidad_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let celular = payload
        .get("celular")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if let Err(e) = sqlx::query("call public.crear_medico($1, $2, $3, $4, $5, $6);")
        .bind(nombre)
        .bind(identificacion)
        .bind(registro_medico)
        .bind(especialidad_id)
        .bind(email)
        .bind(celular)
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

pub async fn delete_medico(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_medico($1)")
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

pub async fn put_medico(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let identificacion = payload
        .get("identificacion")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let registro_medico = payload
        .get("registro_medico")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let especialidad_id = payload
        .get("especialidad_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let celular = payload
        .get("celular")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if let Err(e) = sqlx::query(
        "SELECT * FROM public.modificar_medico($1, $2, $3, $4, $5, $6, $7)",
    )
        .bind(id)
        .bind(nombre)
        .bind(identificacion)
        .bind(registro_medico)
        .bind(especialidad_id)
        .bind(email)
        .bind(celular)
        .fetch_one(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar medico: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

