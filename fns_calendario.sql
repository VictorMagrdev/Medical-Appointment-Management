--public.calendario

create or replace function public.calendario_por_medico(
	p_medico_id int
) returns table (
	v_nombre_medico varchar,
	v_fecha date,
	v_hora time,
	v_nombre_paciente varchar
) as $$
begin
	return query
	select m.nombre, c.fecha, c.hora, p.nombre from public.citas c
	inner join public.medicos m on c.medico_id = m.id
	inner join public.pacientes p on c.paciente_id = p.id
	where c.medico_id = p_medico_id
	and c.estado = 'programada';
end;
$$ language plpgsql;


create or replace function public.calendario_por_especialidad(
	p_especialidad_id int
) returns table (
	v_nombre_medico varchar,
	v_fecha date,
	v_hora time,
	v_nombre_paciente varchar
) as $$
begin
	return query
	select m.nombre, c.fecha, c.hora, p.nombre from public.citas c
	inner join public.medicos m on c.medico_id = m.id
	inner join public.pacientes p on c.paciente_id = p.id
	where m.especialidad_id = p_especialidad_id
		and c.estado = 'programada';
end;
$$ language plpgsql;


create or replace function public.calendario_por_paciente(
	p_paciente_id int
) returns table (
	v_nombre_paciente varchar,
	v_fecha date,
	v_hora time,
	v_nombre_medico varchar
) as $$
begin
	return query
	select p.nombre, c.fecha, c.hora, m.nombre from public.citas c
	inner join public.medicos m on c.medico_id = m.id
	inner join public.pacientes p on c.paciente_id = p.id
	where m.especialidad_id = p_especialidad_id;
end;
$$ language plpgsql;