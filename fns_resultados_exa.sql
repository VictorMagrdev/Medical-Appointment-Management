--public.resultados_examenes


create or replace procedure public.crear_resultado_examen
(
	p_diagnostico text,
	p_posible_tratamiento text,
	p_examen_id int,
	p_medico_id int
)language plpgsql
as $$
begin	

    insert into public.resultados_examenes (diagnostico, posible_tratamiento, examen_id, medico_id)
    values (p_diagnostico, p_posible_tratamiento, p_examen_id, p_medico_id);

exception	
	when foreign_key_violation then
		rollback;
		raise notice 'El examen o medico asociado no existe.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
	
end;
$$;


create or replace function public.resultados_examenes_paciente(
	p_paciente_id int
) returns table (
	v_nombre_paciente varchar,
	v_fecha_examen date,
	v_diagnostico text,
	v_posible_tratamiento text,
	v_nombre_medico varchar
	
) as $$
begin
	return query

	select p.nombre, ex.fecha, re.diagnostico, re.posible_tratamiento, m.nombre from public.examenes ex
	inner join public.resultados_examenes re on ex.id = re.examen_id
	inner join public.historias_clinicas h on ex.historia_clinica_id = h.id
	inner join public.citas c on h.cita_id = c.id
	inner join public.pacientes p on c.paciente_id = p.id
	inner join public.medicos m on re.medico_id = m.id
	where p.id = p_paciente_id
		and ex.estado = 'efectuado';
end;
$$ language plpgsql;



