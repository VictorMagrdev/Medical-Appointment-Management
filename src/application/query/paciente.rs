use crate::infrastructure::data::db::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_pacientes(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_pacientes()")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener pacientes: {e}"),
            ))
        }
    };

    let mut pacientes: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut paciente = HashMap::new();
        paciente.insert("id".to_string(), json!(row.get::<i32, _>("id")));
        paciente.insert("nombre".to_string(), json!(row.get::<String, _>("nombre")));
        paciente.insert(
            "identificacion".to_string(),
            json!(row.get::<String, _>("identificacion")),
        );
        paciente.insert(
            "fecha_nacimiento".to_string(),
            json!(row.get::<String, _>("fecha_nacimiento")),
        );
        paciente.insert("sexo".to_string(), json!(row.get::<String, _>("sexo")));
        paciente.insert(
            "direccion".to_string(),
            json!(row.get::<String, _>("direccion")),
        );
        paciente.insert("email".to_string(), json!(row.get::<String, _>("email")));
        paciente.insert(
            "celular".to_string(),
            json!(row.get::<String, _>("celular")),
        );
        paciente.insert(
            "seguro_id".to_string(),
            json!(row.get::<i32, _>("seguro_id")),
        );
        pacientes.push(paciente);
    }

    Ok(Json(pacientes))
}
