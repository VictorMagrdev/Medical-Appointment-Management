use crate::infrastructure::data::db::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_especialidades(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_especialidades()")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener especialidades: {e}"),
            ))
        }
    };

    let mut especialidades: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut especialidad = HashMap::new();
        especialidad.insert("id".to_string(), json!(row.get::<i32, _>("id")));
        especialidad.insert("nombre".to_string(), json!(row.get::<String, _>("nombre")));
        especialidades.push(especialidad);
    }

    Ok(Json(especialidades))
}
