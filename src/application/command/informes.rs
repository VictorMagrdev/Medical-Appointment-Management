use crate::infrastructure::data::db::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::Value;

pub async fn generar_informe_citas_mes_medico(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let medico_id = payload
        .get("medico_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let mes = payload.get("mes").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let anio = payload.get("anio").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    if let Err(e) = sqlx::query("CALL public.generar_informe_citas_mes_medico($1, $2, $3);")
        .bind(medico_id)
        .bind(mes)
        .bind(anio)
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al generar informe de citas mes y médico: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn generar_informe_citas_pendientes_pacientes(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.generar_informe_citas_pendientes_pacientes();")
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al generar informe de citas pendientes por pacientes: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn generar_informe_medicamentos_entregados(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.generar_informe_medicamentos_entregados();")
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al generar informe de medicamentos entregados: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}

pub async fn generar_informe_examenes_pendientes_pacientes(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("CALL public.generar_informe_examenes_pendientes_pacientes();")
        .execute(&state.get_db_pg())
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error al generar informe de exámenes pendientes por pacientes: {e}"),
        ));
    }

    Ok(StatusCode::CREATED)
}
