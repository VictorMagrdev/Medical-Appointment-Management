use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub async fn post_medicamento(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let principio_activo = payload
        .get("principio_activo")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let forma_farmaceutica = payload
        .get("forma_farmaceutica")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let dosis = payload
        .get("dosis")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let indicaciones_uso = payload
        .get("indicaciones_uso")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let duracion_tratamiento = payload
        .get("duracion_tratamiento")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let historia_clinica_id = payload
        .get("historia_clinica_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    if let Err(e) = sqlx::query(
        "CALL public.crear_medicamento($1, $2, $3, $4, $5, $6, $7, $8);",
    )
    .bind(nombre)
    .bind(principio_activo)
    .bind(forma_farmaceutica)
    .bind(dosis)
    .bind(indicaciones_uso)
    .bind(duracion_tratamiento)
    .bind(estado)
    .bind(historia_clinica_id)
    .execute(&state.get_db_pg())
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear medicamento: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn delete_medicamento(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_medicamento($1);")
        .bind(id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar medicamento: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn put_medicamento(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nombre = payload
        .get("nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let principio_activo = payload
        .get("principio_activo")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let forma_farmaceutica = payload
        .get("forma_farmaceutica")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let dosis = payload
        .get("dosis")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let indicaciones_uso = payload
        .get("indicaciones_uso")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let duracion_tratamiento = payload
        .get("duracion_tratamiento")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let estado = payload
        .get("estado")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let historia_clinica_id = payload
        .get("historia_clinica_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    if let Err(e) = sqlx::query(
        "call public.modificar_medicamento($1, $2, $3, $4, $5, $6, $7, $8, $9);"
    )
        .bind(id)
        .bind(nombre)
        .bind(principio_activo)
        .bind(forma_farmaceutica)
        .bind(dosis)
        .bind(indicaciones_uso)
        .bind(duracion_tratamiento)
        .bind(estado)
        .bind(historia_clinica_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar medicamento: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
