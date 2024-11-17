use crate::application::command::auditoria::{delete_auditoria, get_auditoria_by_id, post_auditoria, put_auditoria_by_id};
use crate::application::command::paciente::{delete_paciente, post_paciente, put_paciente};
use crate::application::query::paciente::get_pacientes;
use crate::infrastructure::data::db::AppState;
use axum::routing::{post, put, delete};
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/pacientes", post(post_paciente).get(get_pacientes))
                .route("/pacientes/:id", put(put_paciente).delete(delete_paciente))
                .route("/auditorias", post(post_auditoria))
                .route("/auditorias/:id", delete(delete_auditoria).put(put_auditoria_by_id)
                    .get(get_auditoria_by_id)),
        )
        .with_state(state)
}
