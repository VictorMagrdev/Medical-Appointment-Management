create sequence public.citas_id_seq 
    start with 100 
    increment by 1 
    minvalue 100 
    maxvalue 999 
    cycle 
    owned by public.citas.id;

create sequence public.seguro_medico_id_seq
    start with 1000
    increment by 10
    minvalue 1000
    maxvalue 999999999999
    owned by public.seguro_medico.id;

create sequence public.historias_clinicas_id_seq
    start with 1
    increment by 4
    minvalue 1
    maxvalue 9223372036854775807
    owned by public.historias_clinicas.id;

create sequence public.medicamentos_id_seq
    start with 1
    increment by 3
    minvalue 1
    maxvalue 999999999 
    owned by public.medicamentos.id;

create sequence public.resultados_examenes_id_seq
    start with 1
    increment by 1
    minvalue 1
    maxvalue 999999999
    owned by public.resultados_examenes.id;

create sequence public.remisiones_medicas_id_seq
    start with 1
    increment by 7
    minvalue 1
    maxvalue 999999999
    owned by public.remisiones_medicas.id;

create sequence public.medicos_id_seq
    start with 3000
    increment by 3
    minvalue 3000
    maxvalue 99999999
    owned by public.medicos.registro_medico;

create sequence public.especialidades_id_seq
    start with 1
    increment by 2
    minvalue 1
    maxvalue 99999
    owned by public.especialidades.id;

create sequence public.informes_id_seq
    start with 1
    increment by 4
    minvalue 1
    maxvalue 9999999999
    owned by public.informes.id;

create sequence public.examenes_id_seq
    start with 1
    increment by 2
    minvalue 1
    maxvalue 9999999999
    owned by public.examenes.id;
