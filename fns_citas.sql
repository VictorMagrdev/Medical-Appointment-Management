--public.citas

create or replace procedure public.crear_cita(
    p_fecha date,
    p_hora time,
    p_motivo varchar,
    p_paciente_id int,
    p_medico_id int
)
language plpgsql
as $$
begin
insert into public.citas (fecha, hora, motivo, estado, paciente_id, medico_id)
values (p_fecha, p_hora, p_motivo, 'programada'::public.estado_cita, p_paciente_id, p_medico_id);

exception
    when unique_violation then
        raise notice 'Ya existe una cita programada con esta combinación de fecha y hora.';

	when foreign_key_violation then
	        raise notice 'El paciente o médico asociado no existe.';
	
	when not_null_violation then
	        raise notice 'Uno de los valores obligatorios es NULL.';
	
	when others then
	        raise notice 'Error inesperado: %', sqlstate;
	end;
$$;


create or replace procedure public.cambiar_estado_cita(
	p_id int,
	p_estado varchar
)
language plpgsql
as $$
begin
	update public.citas
	set estado = p_estado::public.estado_cita
	where id = p_id;
	
	if not found then
		raise exception 'La cita no existe';
	end if;	

end;
$$;


create or replace function public.verificar_disponibilidad_medico()
returns trigger as $$
declare
	v_fecha date;
	v_hora time;
begin 
	if exists(
		select 1 from public.calendario where medico_id = new.medico_id
		and fecha = new.fecha
		and hora = new.hora
	) then
		raise exception 'No es posible crear la cita en esta fecha y hora';
	end if;
	return new;
exception
    when others then
        raise notice 'Error al verificar disponibilidad: %', SQLERRM;
        return null;
end;
$$ language plpgsql;


create trigger tg_verificar_disponibilidad_medico
before insert on public.citas
for each row execute procedure public.verificar_disponibilidad_medico();


create or replace function public.generar_cita_calendario()
returns trigger as $$
declare
begin 
	insert into public.calendario(fecha, hora, medico_id)
	values(new.fecha, new.hora, new.medico_id);
	return new;
exception
	when others then
		raise notice  'Error al generar cita en el calendario: %', SQLERRM;
		return null;
end;
$$ language plpgsql;


create trigger tg_generar_cita_calendario
after insert on public.citas
for each row execute procedure public.generar_cita_calendario();


create or replace function public.actualizar_calendario()
returns trigger as $$
begin
	if new.estado = 'completada' or new.estado = 'cancelada' then
		delete from public.calendario
		where fecha = old.fecha and
		hora = old.hora and
		medico_id = old.medico_id;
		if not found then
			raise exception 'No se encontro la cita en el calendario';
		end if;
	end if; 
	return new;
end;
$$ language plpgsql;
	
create trigger tg_actualizar_calendario
after update on public.citas
for each row execute procedure public.actualizar_calendario();

create or replace function public.validar_cambio_estado()
returns trigger as $$
begin
	if new.estado = 'completada' and old.estado != 'en proceso' then
		raise exception 'No se puede cambiar el estado a "completada" si el estado anterior no era "en proceso"';
	end if; 
	return new;
end;
$$ language plpgsql;
	
create trigger tg_actualizar_calendario
before update on public.citas
for each row execute procedure public.validar_cambio_estado();


--Funciones

create or replace function public.obtener_dia_cita(
	p_cita_id int
)
returns date as $$
begin
    return (select dia from public.citas where id = p_cita_id);
end;
$$ language plpgsql;


create or replace function public.obtener_hora_cita(
	p_cita_id int
)
returns time as $$
begin
    return (select hora from public.citas where id = p_cita_id);
end;
$$ language plpgsql;


create or replace function public.obtener_medico_cita(
    p_cita_id int
)
returns varchar as $$
begin
    return (
        select m.nombre
        from public.medicos m
        join public.citas c on c.medico_id = m.id
        where c.id = p_cita_id
    );
end;
$$ language plpgsql;


create or replace function public.obtener_paciente_cita(
    p_cita_id int
)
returns varchar as $$
begin
    return (
        select p.nombre
        from public.pacientes p
        join public.citas c on c.paciente_id = p.id
        where c.id = p_cita_id
    );
end;
$$ language plpgsql;

create or replace function public.obtener_estado_cita(
    p_cita_id int
)
returns varchar as $$
begin
    return (
        select estado::varchar
        from public.citas
        where id = p_cita_id
    );
end;
$$ language plpgsql;


create or replace function public.obtener_motivo_cita(
    p_cita_id int
)
returns varchar as $$
begin
    return (
        select motivo
        from public.citas
        where id = p_cita_id
    );
end;
$$ language plpgsql;
