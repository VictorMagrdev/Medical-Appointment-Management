use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn post_historia_clinica(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let datos: Value = payload.get("datos").cloned().unwrap_or(Value::Null);
    let cita_id = payload.get("cita_id").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("CALL public.crear_historia_clinica($1, $2);")
        .bind(datos)
        .bind(cita_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear historia clínica: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn delete_historia_clinica(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_historia_clinica($1);")
        .bind(id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar historia clínica: {e}"),
        ));
    }
    Ok(StatusCode::OK)
}

pub async fn put_historia_clinica(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let datos: Value = payload.get("datos").cloned().unwrap_or(Value::Null);
    let cita_id = payload.get("cita_id").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("call public.modificar_historia_clinica($1, $2, $3);")
        .bind(id)
        .bind(datos)
        .bind(cita_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar historia clínica: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
