use crate::infrastructure::data::db::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;
use chrono::NaiveDate;
use serde_json::{json, Value};
use sqlx::{ Row};

pub async fn post_auditoria(
    State(client): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = payload.get("id").and_then(|v| v.as_i64()).unwrap_or(0);

    let fecha = payload
        .get("fecha")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let nombre_paciente = payload
        .get("nombre_paciente")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let nombre_doctor = payload
        .get("nombre_doctor")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let motivo_cita = payload
        .get("motivo_cita")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let diagnostico = payload
        .get("diagnostico")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let medicamentos_recetados = payload
        .get("medicamentos_recetados")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let new_doc = doc! {
        "id": id,
        "fecha": fecha.to_string(),
        "nombre_paciente": nombre_paciente,
        "nombre_doctor": nombre_doctor,
        "motivo_cita": motivo_cita,
        "diagnostico": diagnostico,
        "medicamentos_recetados": medicamentos_recetados,
    };

    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    match auditorias.insert_one(new_doc).await {
        Ok(result) => {
            println!("Documento insertado: {:?}", result.inserted_id);
            Ok(StatusCode::CREATED)
        }
        Err(err) => {
            eprintln!("Error al insertar el documento: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_auditoria(
    State(client): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    match auditorias.delete_one(doc! { "id": id }).await {
        Ok(result) => {
            println!("Documento eliminado: {:?}", result.deleted_count);
            Ok(StatusCode::OK)
        }
        Err(err) => {
            eprintln!("Error al insertar el documento: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn put_auditoria_by_id(
    State(client): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {
    let fecha = payload
        .get("fecha")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let nombre_paciente = payload
        .get("nombre_paciente")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let nombre_doctor = payload
        .get("nombre_doctor")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let motivo_cita = payload
        .get("motivo_cita")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let diagnostico = payload
        .get("diagnostico")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let medicamentos_recetados = payload
        .get("medicamentos_recetados")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    let update = doc! {"$set": doc! {
    "fecha": fecha.to_string(),
    "nombre_paciente": nombre_paciente,
    "nombre_doctor": nombre_doctor,
    "motivo_cita": motivo_cita,
    "diagnostico": diagnostico,
    "medicamentos_recetados": medicamentos_recetados}};

    match auditorias.update_one(doc! { "id": id }, update).await {
        Ok(result) => {
            println!("Documento actualizado: {:?}", result.modified_count);
            Ok(StatusCode::OK)
        }
        Err(err) => {
            eprintln!("Error al insertar el documento: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
/*
    pool: &PgPool,
    cita_id: i32,
    historia_id: i64,
*/
//Para probar
pub async fn obtener_detalles_auditoria(
    State(client): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Value, StatusCode> {
    let cita_id = payload
        .get("seguro_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let historia_id = payload
        .get("historia_id")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let fecha_result = sqlx::query("SELECT public.obtener_dia_cita($1) as fecha")
        .bind(cita_id)
        .fetch_one(&client.get_db_pg())
        .await;

    let fecha = match fecha_result {
        Ok(row) => row.get::<String, _>("fecha"),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let paciente_result = sqlx::query("SELECT public.obtener_paciente_cita($1) as paciente")
        .bind(cita_id)
        .fetch_one(&client.get_db_pg())
        .await;

    let nombre_paciente = match paciente_result {
        Ok(row) => row.get::<String, _>("paciente"),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let doctor_result = sqlx::query("SELECT public.obtener_medico_cita($1) as doctor")
        .bind(cita_id)
        .fetch_one(&client.get_db_pg())
        .await;

    let nombre_doctor = match doctor_result {
        Ok(row) => row.get::<String, _>("doctor"),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let diagnostico_result = sqlx::query("SELECT public.obtener_diagnostico_historia($1) as diagnostico")
        .bind(historia_id)
        .fetch_one(&client.get_db_pg())
        .await;

    let diagnostico = match diagnostico_result {
        Ok(row) => row.get::<String, _>("diagnostico"),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let medicamentos_result = sqlx::query("SELECT nombre FROM public.obtener_nombre_medicamentos()")
        .fetch_all(&client.get_db_pg())
        .await;

    let medicamentos_recetados = match medicamentos_result {
        Ok(rows) => {
            let medicamentos: Vec<String> = rows.into_iter()
                .map(|row| row.get::<String, _>("nombre"))
                .collect();
            medicamentos.join(", ") // Unimos los nombres de los medicamentos con una coma
        }
        Err(_) => String::new(),
    };

    let motivo_result = sqlx::query("SELECT public.obtener_motivo_cita($1) as motivo")
        .bind(cita_id)
        .fetch_one(&client.get_db_pg())
        .await;

    let motivo_cita = match motivo_result {
        Ok(row) => row.get::<String, _>("motivo"),
        Err(_) => String::new(),
    };

    let auditoria = json!({
        "fecha": fecha,
        "nombre_paciente": nombre_paciente,
        "nombre_doctor": nombre_doctor,
        "motivo_cita": motivo_cita,
        "diagnostico": diagnostico,
        "medicamentos_recetados": medicamentos_recetados,
    });

    Ok(auditoria)
}
/*
    pool: &PgPool,
    cita_id: i32,
    historia_id: i64,
    client: &AppState,
    payload: Value,
*/
pub async fn registrar_auditoria_y_detalles(
    State(client): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, StatusCode> {

    let auditoria_detalles = match obtener_detalles_auditoria(State(client.clone()), Json(payload.clone())).await {
        Ok(details) => details,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut auditoria_payload = auditoria_detalles;
    auditoria_payload["id"] = payload.get("id").cloned().unwrap_or(Value::Null);

    let auditoria_result = post_auditoria(State(client), Json(auditoria_payload)).await;

    match auditoria_result {
        Ok(response) => Ok(response),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
