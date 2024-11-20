--public.citas

// CREAR CITA
create or replace procedure public.crear_cita(
    p_fecha date,
    p_hora time,
    p_motivo varchar,
    estado estado_cita,
    paciente_id int,
    medico_id int
)
language plpgsql
as $$
begin	

    insert into public.citas (fecha, hora, motivo, 'programada', paciente_id, medico_id)
    values (p_nombre);

exception
	when unique_violation then
		rollback;
		raise notice 'El nombre de la especialidad ya existe en el sistema.';
	
	when foreign_key_violation then
		rollback;
		raise notice 'El paciente o medico asociado no existe.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
	
end;
$$;

create or replace procedure public.cambiar_estado_cita(
	p_id int,
	p_estado estado_cita
)
language plpgsql
as $$
begin
	update public.citas
	set estado = p_estado
	where id = p_id;
	
	if not found then
		raise exception 'La cita no existe';
	end if;	

end;
$$;

// TRIGGERS CITAS

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
	when other then
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
end


