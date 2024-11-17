use crate::application::command::auditoria::{
    delete_auditoria, get_auditoria_by_id, post_auditoria, put_auditoria_by_id,
};
use crate::application::command::examenes::{delete_examen, post_examen, put_examen};
use crate::application::command::historias_clinicas::{
    delete_historia_clinica, post_historia_clinica, put_historia_clinica,
};
use crate::application::command::medicamentos::{
    delete_medicamento, post_medicamento, put_medicamento,
};
use crate::application::command::medicos::{delete_medico, put_medico};
use crate::application::command::paciente::{delete_paciente, post_paciente, put_paciente};
use crate::application::command::seguro_medico::post_seguro_medico;
use crate::application::query::paciente::get_pacientes;
use crate::infrastructure::data::db::AppState;
use axum::routing::{delete, post, put};
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/pacientes", post(post_paciente).get(get_pacientes))
                .route("/pacientes/:id", put(put_paciente).delete(delete_paciente))
                .route("/auditorias", post(post_auditoria))
                .route(
                    "/auditorias/:id",
                    delete(delete_auditoria)
                        .put(put_auditoria_by_id)
                        .get(get_auditoria_by_id),
                )
                .route("/examenes", post(post_examen))
                .route("/examenes/:id", put(put_examen).delete(delete_examen))
                .route("/historias-clinicas", post(post_historia_clinica))
                .route(
                    "/historias-clinicas/:id",
                    put(put_historia_clinica).delete(delete_historia_clinica),
                )
                .route("/medicamentos", post(post_medicamento))
                .route(
                    "/medicamentos/:id",
                    put(put_medicamento).delete(delete_medicamento),
                )
                .route("/seguros-medicos", post(post_seguro_medico))
                .route(
                    "/seguros-medicos/:id",
                    put(put_medico).delete(delete_medico),
                ),
        )
        .with_state(state)
}
