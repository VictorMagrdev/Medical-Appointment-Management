use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn post_especialidad(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .ok_or((
            StatusCode::BAD_REQUEST,
            "El nombre de la especialidad es obligatorio".to_string(),
        ))?
        .to_string();

    if let Err(e) = sqlx::query("CALL public.crear_especialidad($1)")
        .bind(nombre)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear la especialidad: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn delete_especialidad(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_especialidad($1)")
        .bind(id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar la especialidad: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn put_especialidad(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .ok_or((
            StatusCode::BAD_REQUEST,
            "El nombre de la especialidad es obligatorio".to_string(),
        ))?
        .to_string();

    if let Err(e) = sqlx::query("CALL public.modificar_especialidad($1, $2)")
        .bind(id)
        .bind(nombre)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar la especialidad: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
