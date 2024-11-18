use crate::infrastructure::data::db::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

pub async fn get_medicamentos(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let rows = match sqlx::query("SELECT * FROM public.obtener_medicamentos();")
        .fetch_all(&state.get_db_pg())
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener medicamentos: {e}"),
            ));
        }
    };

    let mut medicamentos: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut medicamento = HashMap::new();
        medicamento.insert("id".to_string(), json!(row.get::<i32, _>("id")));
        medicamento.insert("nombre".to_string(), json!(row.get::<String, _>("nombre")));
        medicamento.insert(
            "principio_activo".to_string(),
            json!(row.get::<String, _>("principio_activo")),
        );
        medicamento.insert(
            "forma_farmaceutica".to_string(),
            json!(row.get::<String, _>("forma_farmaceutica")),
        );
        medicamento.insert("dosis".to_string(), json!(row.get::<String, _>("dosis")));
        medicamento.insert(
            "indicaciones_uso".to_string(),
            json!(row.get::<String, _>("indicaciones_uso")),
        );
        medicamento.insert(
            "duracion_tratamiento".to_string(),
            json!(row.get::<String, _>("duracion_tratamiento")),
        );
        medicamento.insert("estado".to_string(), json!(row.get::<String, _>("estado")));
        medicamento.insert(
            "historia_clinica_id".to_string(),
            json!(row.get::<i32, _>("historia_clinica_id")),
        );
        medicamentos.push(medicamento);
    }

    Ok(Json(medicamentos))
}
