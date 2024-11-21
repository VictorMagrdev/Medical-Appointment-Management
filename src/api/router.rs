use crate::application::command::auditoria::{
    delete_auditoria, post_auditoria, put_auditoria_by_id,
};
use crate::application::command::examenes::{delete_examen, post_examen, put_examen};
use crate::application::command::historias_clinicas::{
    delete_historia_clinica, post_historia_clinica, put_historia_clinica,
};
use crate::application::command::medicamentos::{
    delete_medicamento, post_medicamento, put_medicamento,
};
use crate::application::command::medicos::{delete_medico, post_medico, put_medico};
use crate::application::command::paciente::{delete_paciente, post_paciente, put_paciente};
use crate::application::command::seguro_medico::{
    delete_seguro_medico, post_seguro_medico, put_seguro_medico,
};
use crate::application::query::paciente::get_pacientes;
use crate::infrastructure::data::db::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;
use crate::application::command::citas::{post_cita, put_estado_cita};
use crate::application::command::especialidades::{delete_especialidad, post_especialidad, put_especialidad};
use crate::application::query::auditoria::get_auditoria_by_id;
use crate::application::query::calendario::{get_calendario_por_especialidad, get_calendario_por_medico, get_calendario_por_paciente};
use crate::application::query::especialidades::get_especialidades;
use crate::application::query::examenes::get_examenes;
use crate::application::query::historias_clinicas::get_historias_clinicas;
use crate::application::query::medicamentos::get_medicamentos;
use crate::application::query::medicos::get_medicos;
use crate::application::query::seguro_medico::get_seguros_medicos;

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
                .route("/examenes", post(post_examen).get(get_examenes))
                .route("/examenes/:id", put(put_examen).delete(delete_examen))
                .route("/historias-clinicas", post(post_historia_clinica).get(get_historias_clinicas))
                .route(
                    "/historias-clinicas/:id",
                    put(put_historia_clinica).delete(delete_historia_clinica),
                )
                .route("/medicamentos", post(post_medicamento).get(get_medicamentos))
                .route(
                    "/medicamentos/:id",
                    put(put_medicamento).delete(delete_medicamento),
                )
                .route("/seguros-medicos", post(post_seguro_medico).get(get_seguros_medicos))
                .route(
                    "/seguros-medicos/:id",
                    put(put_seguro_medico).delete(delete_seguro_medico),
                )
                .route("/medicos", post(post_medico).get(get_medicos))
                .route("/medicos/:id", put(put_medico).delete(delete_medico))
                .route("/calendario/medico",get(get_calendario_por_medico))
                .route("/calendario/especialidad",get(get_calendario_por_especialidad))
                .route("/calendario/paciente",get(get_calendario_por_paciente))
                .route("/especialidades",get(get_especialidades).post(post_especialidad))
                .route("/especialidades/:id",put(put_especialidad).delete(delete_especialidad))
                .route("/cita",post(post_cita))
                .route("/cita/:id",put(put_estado_cita)),
        )
        .with_state(state)
}
