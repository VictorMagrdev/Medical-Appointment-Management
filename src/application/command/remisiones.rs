use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde_json::Value;

pub async fn create_medical_referral(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let fecha = payload.get("fecha").and_then(|v| v.as_str()).unwrap_or("");
    let motivo_remision = payload
        .get("motivo_remision")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let medico_id = payload
        .get("medico_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let historia_clinica_id = payload
        .get("historia_clinica_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("CALL public.crear_remision_medica($1, $2, $3, $4)")
        .bind(fecha)
        .bind(motivo_remision)
        .bind(medico_id)
        .bind(historia_clinica_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al crear la remisión médica: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn update_medical_referral(
    State(state): State<AppState>,
    Path(remision_id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let fecha = payload.get("fecha").and_then(|v| v.as_str()).unwrap_or("");
    let motivo_remision = payload
        .get("motivo_remision")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let medico_id = payload
        .get("medico_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let historia_clinica_id = payload
        .get("historia_clinica_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("CALL public.actualizar_remision_medica($1, $2, $3, $4, $5)")
        .bind(remision_id)
        .bind(fecha)
        .bind(motivo_remision)
        .bind(medico_id)
        .bind(historia_clinica_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al actualizar la remisión médica: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn delete_medical_referral(
    State(state): State<AppState>,
    Path(remision_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.eliminar_remision_medica($1)")
        .bind(remision_id)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar la remisión médica: {e}"),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_xml_node(
    State(state): State<AppState>,
    Path(remision_id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nodo_nombre = payload
        .get("nodo_nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let nodo_valor = payload
        .get("nodo_valor")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if let Err(e) = sqlx::query("CALL public.agregar_nodo_xml($1, $2, $3)")
        .bind(remision_id)
        .bind(nodo_nombre)
        .bind(nodo_valor)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al agregar el nodo XML: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn modify_xml_node(
    State(state): State<AppState>,
    Path(remision_id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nodo_nombre = payload
        .get("nodo_nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let nuevo_valor = payload
        .get("nuevo_valor")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if let Err(e) = sqlx::query("CALL public.modificar_nodo_xml($1, $2, $3)")
        .bind(remision_id)
        .bind(nodo_nombre)
        .bind(nuevo_valor)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al modificar el nodo XML: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn delete_xml_node(
    State(state): State<AppState>,
    Path(remision_id): Path<i32>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let nodo_nombre = payload
        .get("nodo_nombre")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if let Err(e) = sqlx::query("CALL public.eliminar_nodo_xml($1, $2)")
        .bind(remision_id)
        .bind(nodo_nombre)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al eliminar el nodo XML: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
