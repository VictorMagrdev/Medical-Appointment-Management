create table public.citas (
    id int primary key default nextval('public.citas_id_seq'),
    fecha date not null,
    hora time not null,
    motivo varchar(255),
    estado estado_cita not null,
    paciente_id int references public.pacientes(id),
    medico_id int references medicos(id),
);


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

    insert into public.citas (fecha, hora, motivo, estado, paciente_id, medico_id)
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
end;
$$ language plpgsql;


create trigger tg_generar_cita_calendario
after insert on public.citas
for each row execute procedure public.generar_cita_calendario();





