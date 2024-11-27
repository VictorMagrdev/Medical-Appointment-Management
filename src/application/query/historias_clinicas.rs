use crate::infrastructure::data::db::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_historias_clinicas(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_historias_clinicas();")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener historias clínicas: {e}"),
            ));
        }
    };

    let mut historias_clinicas: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut historia_clinica = HashMap::new();
        historia_clinica.insert("id".to_string(), json!(row.get::<i64, _>("id")));
        historia_clinica.insert("datos".to_string(), json!(row.get::<Value, _>("datos")));
        historia_clinica.insert("cita_id".to_string(), json!(row.get::<i32, _>("cita_id")));
        historias_clinicas.push(historia_clinica);
    }

    Ok(Json(historias_clinicas))
}
