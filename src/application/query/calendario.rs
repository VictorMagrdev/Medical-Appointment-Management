use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_calendario_por_medico(
    State(state): State<AppState>,
    Path(medico_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.calendario_por_medico($1)")
        .bind(medico_id)
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el calendario por médico: {e}"),
            ))
        }
    };

    let mut calendario: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut cita = HashMap::new();
        cita.insert(
            "nombre_medico".to_string(),
            json!(row.get::<String, _>("v_nombre_medico")),
        );
        cita.insert("fecha".to_string(), json!(row.get::<String, _>("v_fecha")));
        cita.insert("hora".to_string(), json!(row.get::<String, _>("v_hora")));
        cita.insert(
            "nombre_paciente".to_string(),
            json!(row.get::<String, _>("v_nombre_paciente")),
        );
        calendario.push(cita);
    }

    Ok(Json(calendario))
}

pub async fn get_calendario_por_especialidad(
    State(state): State<AppState>,
    Path(especialidad_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.calendario_por_especialidad($1)")
        .bind(especialidad_id)
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el calendario por especialidad: {e}"),
            ))
        }
    };

    let mut calendario: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut cita = HashMap::new();
        cita.insert(
            "nombre_medico".to_string(),
            json!(row.get::<String, _>("v_nombre_medico")),
        );
        cita.insert("fecha".to_string(), json!(row.get::<String, _>("v_fecha")));
        cita.insert("hora".to_string(), json!(row.get::<String, _>("v_hora")));
        cita.insert(
            "nombre_paciente".to_string(),
            json!(row.get::<String, _>("v_nombre_paciente")),
        );
        calendario.push(cita);
    }

    Ok(Json(calendario))
}

pub async fn get_calendario_por_paciente(
    State(state): State<AppState>,
    Path(paciente_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.calendario_por_paciente($1)")
        .bind(paciente_id)
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el calendario por paciente: {e}"),
            ))
        }
    };

    let mut calendario: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut cita = HashMap::new();
        cita.insert(
            "nombre_paciente".to_string(),
            json!(row.get::<String, _>("v_nombre_paciente")),
        );
        cita.insert("fecha".to_string(), json!(row.get::<String, _>("v_fecha")));
        cita.insert("hora".to_string(), json!(row.get::<String, _>("v_hora")));
        cita.insert(
            "nombre_medico".to_string(),
            json!(row.get::<String, _>("v_nombre_medico")),
        );
        calendario.push(cita);
    }

    Ok(Json(calendario))
}
