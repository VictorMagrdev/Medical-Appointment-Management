use crate::infrastructure::data::db::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bson::doc;

pub async fn get_auditoria_by_id(
    State(client): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    let result = auditorias.find_one(doc! {"id": id}).await;

    match result {
        Ok(Some(resp)) => {
            println!("Documento encontrado: {:?}", resp);
            Ok(Json(resp))
        }
        Ok(None) => {
            println!("Documento no encontrado.");
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            eprintln!("Error al insertar el documento: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_auditoria_by_date(
    State(client): State<AppState>,
    Path(fecha): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    let result = auditorias.find_one(doc! {"fecha": fecha}).await;

    match result {
        Ok(Some(resp)) => {
            println!("fecha encontrada: {:?}", resp);
            Ok(Json(resp))
        }
        Ok(None) => {
            println!("fecha no encontrado.");
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            eprintln!("Error al insertar el fecha: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_auditoria_by_pacient(
    State(client): State<AppState>,
    Path(paciente): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    let result = auditorias.find_one(doc! {"nombre_paciente": paciente}).await;

    match result {
        Ok(Some(resp)) => {
            println!("paciente encontrada: {:?}", resp);
            Ok(Json(resp))
        }
        Ok(None) => {
            println!("paciente no encontrado.");
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            eprintln!("Error al insertar el paciente: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn get_auditoria_by_doctor(
    State(client): State<AppState>,
    Path(doctor): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let auditorias = client
        .get_db_mongo()
        .database("doctorya")
        .collection::<bson::Document>("auditoria");

    let result = auditorias.find_one(doc! {"nombre_doctor": doctor}).await;

    match result {
        Ok(Some(resp)) => {
            println!("paciente encontrada: {:?}", resp);
            Ok(Json(resp))
        }
        Ok(None) => {
            println!("paciente no encontrado.");
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            eprintln!("Error al insertar el paciente: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}