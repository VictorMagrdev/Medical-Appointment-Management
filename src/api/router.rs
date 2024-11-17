use crate::application::command::paciente::{delete_paciente, post_paciente, put_paciente};
use crate::application::query::paciente::get_pacientes;
use crate::infrastructure::data::db_pg::AppState;
use axum::routing::{post, put};
use axum::Router;
use crate::application::command::auditoria::post_auditoria;
use crate::infrastructure::data::db_mongo::ClientState;

pub fn create_router(state: AppState, client: ClientState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/pacientes", post(post_paciente).get(get_pacientes))
                .route("/pacientes/:id", put(put_paciente).delete(delete_paciente))
                .route("/auditorias", post(post_auditoria)),
        )
        .with_state(state).with_state(client)
}
