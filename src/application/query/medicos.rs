use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_medicos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_medicos()")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener médicos: {e}"),
            ));
        }
    };

    let mut medicos: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut medico = HashMap::new();
        medico.insert("id".to_string(), json!(row.get::<i32, _>("id")));
        medico.insert("nombre".to_string(), json!(row.get::<String, _>("nombre")));
        medico.insert(
            "identificacion".to_string(),
            json!(row.get::<String, _>("identificacion")),
        );
        medico.insert(
            "registro_medico".to_string(),
            json!(row.get::<String, _>("registro_medico")),
        );
        medico.insert(
            "especialidad_id".to_string(),
            json!(row.get::<i32, _>("especialidad_id")),
        );
        medico.insert("email".to_string(), json!(row.get::<String, _>("email")));
        medico.insert(
            "celular".to_string(),
            json!(row.get::<String, _>("celular")),
        );
        medicos.push(medico);
    }

    Ok(Json(medicos))
}

pub async fn obtener_medico(
    State(state): State<AppState>,
    Path(medico_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_medico($1) as medico")
        .bind(medico_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el médico: {e}"),
            ));
        }
    };

    let medico: Option<String> = row.get("medico");

    Ok(Json(json!({ "medico": medico.unwrap_or_default() })))
}

pub async fn obtener_especialidad_medico(
    State(state): State<AppState>,
    Path(medico_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_especialidad_medico($1) as especialidad")
        .bind(medico_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener la especialidad del médico: {e}"),
            ));
        }
    };

    let especialidad: Option<String> = row.get("especialidad");

    Ok(Json(
        json!({ "especialidad": especialidad.unwrap_or_default() }),
    ))
}

pub async fn obtener_identificacion_medico(
    State(state): State<AppState>,
    Path(medico_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_identificacion_medico($1) as identificacion")
        .bind(medico_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener la identificación del médico: {e}"),
            ));
        }
    };

    let identificacion: Option<String> = row.get("identificacion");

    Ok(Json(
        json!({ "identificacion": identificacion.unwrap_or_default() }),
    ))
}
