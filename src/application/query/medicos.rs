use crate::infrastructure::data::db::AppState;
use axum::extract::State;
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
