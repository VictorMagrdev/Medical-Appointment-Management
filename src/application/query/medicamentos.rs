use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
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
    println!("Rows obtenidos: {:?}", rows);

    let mut medicamentos: Vec<HashMap<String, Value>> = Vec::new();

    for row in rows {
        let mut medicamento = HashMap::new();
        medicamento.insert("id".to_string(), json!(row.get::<i64, _>("id")));
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
            json!(row.get::<i64, _>("historia_clinica_id")),
        );
        medicamentos.push(medicamento);
    }

    Ok(Json(medicamentos))
}

pub async fn obtener_nombre_medicamento(
    State(state): State<AppState>,
    Path(medicamento_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_nombre_medicamento($1) as nombre")
        .bind(medicamento_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el nombre del medicamento: {e}"),
            ));
        }
    };

    let nombre: Option<String> = row.get("nombre");

    Ok(Json(json!({ "nombre": nombre.unwrap_or_default() })))
}

pub async fn obtener_forma_farmaceutica_medicamento(
    State(state): State<AppState>,
    Path(medicamento_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query(
        "SELECT public.obtener_forma_farmaceutica_medicamento($1) as forma_farmaceutica",
    )
    .bind(medicamento_id)
    .fetch_one(&state.get_db_pg())
    .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener la forma farmacéutica del medicamento: {e}"),
            ));
        }
    };

    let forma_farmaceutica: Option<String> = row.get("forma_farmaceutica");

    Ok(Json(
        json!({ "forma_farmaceutica": forma_farmaceutica.unwrap_or_default() }),
    ))
}

pub async fn obtener_estado_medicamento(
    State(state): State<AppState>,
    Path(medicamento_id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let row = match sqlx::query("SELECT public.obtener_estado_medicamento($1) as estado")
        .bind(medicamento_id)
        .fetch_one(&state.get_db_pg())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error al obtener el estado del medicamento: {e}"),
            ));
        }
    };

    let estado: Option<String> = row.get("estado");

    Ok(Json(json!({ "estado": estado.unwrap_or_default() })))
}
