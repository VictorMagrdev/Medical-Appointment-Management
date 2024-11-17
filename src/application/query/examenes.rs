use std::collections::HashMap;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::{json, Value};
use sqlx::Row;
use crate::infrastructure::data::db::AppState;

pub async fn get_examenes(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_examenes();")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener examenes: {e}"),
            ));
        }
    };

    let mut examenes: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut examen = HashMap::new();
        examen.insert("id".to_string(), json!(row.get::<i32, _>("id")));
        examen.insert("nombre".to_string(), json!(row.get::<String, _>("nombre")));
        examen.insert("costo".to_string(), json!(row.get::<f32, _>("costo")));
        examen.insert(
            "cubre_seguro".to_string(),
            json!(row.get::<bool, _>("cubre_seguro")),
        );
        examen.insert(
            "fecha_realizacion".to_string(),
            json!(row.get::<String, _>("fecha_realizacion")),
        );
        examen.insert("estado".to_string(), json!(row.get::<String, _>("estado")));
        examen.insert(
            "historia_clinica_id".to_string(),
            json!(row.get::<i32, _>("historia_clinica_id")),
        );
        examenes.push(examen);
    }

    Ok(Json(examenes))
}

