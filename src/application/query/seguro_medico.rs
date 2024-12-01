use crate::infrastructure::data::db::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_seguros_medicos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_seguros_medicos()")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener seguros médicos: {e}"),
            ));
        }
    };

    let mut seguros_medicos: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut seguro_medico = HashMap::new();
        seguro_medico.insert("id".to_string(), json!(row.get::<i64, _>("v_id")));
        seguro_medico.insert(
            "nombre".to_string(),
            json!(row.get::<String, _>("v_nombre")),
        );
        seguro_medico.insert("tipo".to_string(), json!(row.get::<String, _>("v_tipo")));
        seguro_medico.insert(
            "fecha_inicio".to_string(),
            json!(row.get::<chrono::NaiveDate, _>("v_fecha_inicio")),
        );
        seguro_medico.insert(
            "fecha_final".to_string(),
            json!(row.get::<chrono::NaiveDate, _>("v_fecha_final")),
        );
        seguro_medico.insert(
            "celular_contacto".to_string(),
            json!(row.get::<String, _>("v_celular_contacto")),
        );
        seguros_medicos.push(seguro_medico);
    }

    Ok(Json(seguros_medicos))
}
