use crate::infrastructure::data::db::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;
use chrono::NaiveDate;
use serde_json::Value;

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
