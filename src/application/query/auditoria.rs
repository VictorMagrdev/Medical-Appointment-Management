use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use bson::doc;
use crate::infrastructure::data::db::AppState;

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
