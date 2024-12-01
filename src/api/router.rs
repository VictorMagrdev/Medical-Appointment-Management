use crate::application::command::auditoria::{
    delete_auditoria, post_auditoria, put_auditoria_by_id,
};
use crate::application::command::citas::{post_cita, put_estado_cita};
use crate::application::command::especialidades::{
    delete_especialidad, post_especialidad, put_especialidad,
};
use crate::application::command::examenes::{delete_examen, post_examen, put_examen};
use crate::application::command::historias_clinicas::{
    delete_historia_clinica, post_historia_clinica, put_historia_clinica,
};
use crate::application::command::informes::{
    generar_informe_citas_mes_medico, generar_informe_citas_pendientes_pacientes,
    generar_informe_examenes_pendientes_pacientes, generar_informe_medicamentos_entregados,
};
use crate::application::command::medicamentos::{
    delete_medicamento, post_medicamento, put_medicamento,
};
use crate::application::command::medicos::{delete_medico, post_medico, put_medico};
use crate::application::command::paciente::{delete_paciente, post_paciente, put_paciente};
use crate::application::command::seguro_medico::{
    delete_seguro_medico, post_seguro_medico, put_seguro_medico,
};
use crate::application::query::auditoria::get_auditoria_by_id;
use crate::application::query::calendario::{
    get_calendario_por_especialidad, get_calendario_por_medico, get_calendario_por_paciente,
};
use crate::application::query::citas::{obtener_dia_cita, obtener_hora_cita, obtener_medico_cita};
use crate::application::query::especialidades::get_especialidades;
use crate::application::query::examenes::get_examenes;
use crate::application::query::historias_clinicas::get_historias_clinicas;
use crate::application::query::medicamentos::{
    get_medicamentos, obtener_estado_medicamento, obtener_forma_farmaceutica_medicamento,
    obtener_nombre_medicamento,
};
use crate::application::query::medicos::{
    get_medicos, obtener_especialidad_medico, obtener_identificacion_medico, obtener_medico,
};
use crate::application::query::paciente::{
    get_pacientes, obtener_identificacion_paciente, obtener_paciente,
};
use crate::application::query::seguro_medico::get_seguros_medicos;
use crate::infrastructure::data::db::AppState;
use axum::routing::{get, post, put};
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/pacientes", post(post_paciente).get(get_pacientes))
                .route(
                    "/pacientes/:id",
                    get(obtener_paciente)
                        .put(put_paciente)
                        .delete(delete_paciente),
                )
                .route(
                    "/pacientes/identificacion/:id",
                    get(obtener_identificacion_paciente),
                )
                .route("/auditorias", post(post_auditoria))
                .route(
                    "/auditorias/:id",
                    get(get_auditoria_by_id)
                        .put(put_auditoria_by_id)
                        .delete(delete_auditoria),
                )
                .route("/examenes", post(post_examen).get(get_examenes))
                .route("/examenes/:id", put(put_examen).delete(delete_examen))
                .route(
                    "/historias-clinicas",
                    post(post_historia_clinica).get(get_historias_clinicas),
                )
                .route(
                    "/historias-clinicas/:id",
                    put(put_historia_clinica).delete(delete_historia_clinica),
                )
                .route(
                    "/medicamentos",
                    post(post_medicamento).get(get_medicamentos),
                )
                .route(
                    "/medicamentos/:id",
                    get(obtener_nombre_medicamento)
                        .put(put_medicamento)
                        .delete(delete_medicamento),
                )
                .route(
                    "/medicamentos/formula/:id",
                    get(obtener_forma_farmaceutica_medicamento),
                )
                .route("/medicamentos/estado/:id", get(obtener_estado_medicamento))
                .route(
                    "/seguros-medicos",
                    post(post_seguro_medico).get(get_seguros_medicos),
                )
                .route(
                    "/seguros-medicos/:id",
                    put(put_seguro_medico).delete(delete_seguro_medico),
                )
                .route("/medicos", post(post_medico).get(get_medicos))
                .route(
                    "/medicos/:id",
                    get(obtener_medico).put(put_medico).delete(delete_medico),
                )
                .route(
                    "/medicos/especialidad/:id",
                    get(obtener_especialidad_medico),
                )
                .route(
                    "/medicos/identificacion/:id",
                    get(obtener_identificacion_medico),
                )
                .route("/calendario/medico/:id", get(get_calendario_por_medico))
                .route(
                    "/calendario/especialidad/:id",
                    get(get_calendario_por_especialidad),
                )
                .route("/calendario/paciente/:id", get(get_calendario_por_paciente))
                .route(
                    "/especialidades",
                    post(post_especialidad).get(get_especialidades),
                )
                .route(
                    "/especialidades/:id",
                    put(put_especialidad).delete(delete_especialidad),
                )
                .route("/cita", post(post_cita))
                .route("/cita/:id", put(put_estado_cita))
                .route("/cita/dia/:id", get(obtener_dia_cita))
                .route("/cita/hora/:id", get(obtener_hora_cita))
                .route("/cita/medico/:id", get(obtener_medico_cita))
                .route(
                    "/informes/citas-mes/",
                    post(generar_informe_citas_mes_medico),
                )
                .route(
                    "/informes/citas-pendientes/",
                    post(generar_informe_citas_pendientes_pacientes),
                )
                .route(
                    "/informes/medicamentos-entregados/",
                    post(generar_informe_medicamentos_entregados),
                )
                .route(
                    "/informes/examenes-pendientes/",
                    post(generar_informe_examenes_pendientes_pacientes),
                ),
        )
        .with_state(state)
}
