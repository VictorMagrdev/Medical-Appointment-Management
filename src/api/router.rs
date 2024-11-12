use crate::application::command::paciente::{delete_paciente, post_paciente, put_paciente};
use crate::application::query::paciente::get_pacientes;
use crate::infrastructure::data::db::AppState;
use axum::routing::{post, put};
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/pacientes", post(post_paciente).get(get_pacientes))
                .route("/pacientes/:id", put(put_paciente).delete(delete_paciente)),
        )
        .with_state(state)
}
