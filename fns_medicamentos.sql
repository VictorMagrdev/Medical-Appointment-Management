-- Crear Medicamento
create or replace procedure public.crear_medicamento(
    p_nombre varchar,
    p_principio_activo varchar,
    p_forma_farmaceutica varchar,
    p_dosis varchar,
    p_indicaciones_uso text,
    p_duracion_tratamiento varchar,
    p_estado varchar,
    p_historia_clinica_id int
)
language plpgsql
as $$
begin
    insert into public.medicamentos (nombre, principio_activo, forma_farmaceutica, dosis, indicaciones_uso, duracion_tratamiento, estado, historia_clinica_id)
    values (p_nombre, p_principio_activo, p_forma_farmaceutica::public.forma_farmaceutica, p_dosis, p_indicaciones_uso, p_duracion_tratamiento,
			 p_estado::estado_medicamento, p_historia_clinica_id);

exception
    when foreign_key_violation then
        raise notice 'La historia clínica asociada no existe.';
    
    when null_value_not_allowed then
        raise notice 'Uno de los valores obligatorios es NULL.';
    
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- Eliminar Medicamento
create or replace procedure public.eliminar_medicamento(p_id int)
language plpgsql
as $$
begin
    delete from public.medicamentos where id = p_id;

    if not found then
        raise exception 'Error: El medicamento con ID % no existe', p_id;
    end if;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- Modificar Medicamento
create or replace procedure public.modificar_medicamento(
    p_id int,
    p_nombre varchar,
    p_principio_activo varchar,
    p_forma_farmaceutica varchar,
    p_dosis varchar,
    p_indicaciones_uso text,
    p_duracion_tratamiento varchar,
    p_estado varchar,
    p_historia_clinica_id int
)
language plpgsql
as $$
begin
    update public.medicamentos
    set nombre = p_nombre,
        principio_activo = p_principio_activo,
        forma_farmaceutica = p_forma_farmaceutica::public.forma_farmaceutica,
        dosis = p_dosis,
        indicaciones_uso = p_indicaciones_uso,
        duracion_tratamiento = p_duracion_tratamiento,
        estado = p_estado::public.estado_medicamento,
        historia_clinica_id = p_historia_clinica_id
    where id = p_id;

    if not found then
        raise exception 'Error: El medicamento con ID % no existe', p_id;
    end if;

exception
    when foreign_key_violation then
        raise notice 'La historia clínica asociada no existe.';
    
    when null_value_not_allowed then
        raise notice 'Uno de los valores obligatorios es NULL.';
    
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- Obtener Medicamentos
create or replace function public.obtener_medicamentos()
returns table(
    v_id int,
    v_nombre varchar,
    v_principio_activo varchar,
    v_forma_farmaceutica varchar,
    v_dosis varchar,
    v_indicaciones_uso text,
    v_duracion_tratamiento varchar,
    v_estado varchar,
    v_historia_clinica_id int
)
language plpgsql
as $$
begin
    if not exists (select 1 from public.medicamentos) then
        raise exception 'No se encontraron registros en la tabla de medicamentos.';
    end if;

    return query select id, nombre, principio_activo, forma_farmaceutica::varchar, dosis, indicaciones_uso,
					 duracion_tratamiento, estado::varchar, historia_clinica_id from public.medicamentos;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

create or replace function public.obtener_nombre_medicamentos()
returns table(
    v_nombre varchar
)
language plpgsql
as $$
begin
    if not exists (select 1 from public.medicamentos) then
        raise exception 'No se encontraron registros en la tabla de medicamentos.';
    end if;

    return query select nombre from public.medicamentos;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;



create or replace function public.obtener_nombre_medicamento(
    p_medicamento_id int
)
returns varchar as $$
begin
    return (
        select nombre from public.medicamentos
		where id = p_medicamento_id;
    );
end;
$$ language plpgsql;


create or replace function public.obtener_forma_farmaceutica_medicamento(
    p_medicamento_id int
)
returns varchar as $$
begin
    return (
        select forma_farmaceutica::varchar from public.medicamentos
		where id = p_medicamento_id;
    );
end;
$$ language plpgsql;



create or replace function public.obtener_estado_medicamento(
    p_medicamento_id int
)
returns varchar as $$
begin
    return (
        select estado::varchar from public.medicamentos
		where id = p_medicamento_id;
    );
end;
$$ language plpgsql;


-- Triggers

create or replace function public.validar_medicamento_unico()
returns trigger as $$
begin
    if exists (
        select 1
        from public.medicamentos
        where nombre = new.nombre
          and historia_clinica_id = new.historia_clinica_id
    ) then
        raise exception 'El medicamento "%", ya está asignado a la historia clínica %',
            new.nombre, new.historia_clinica_id;
    end if;
    return new;
end;
$$ language plpgsql;

create trigger trg_validar_medicamento_unico
before insert or update on public.medicamentos
for each row execute function public.validar_medicamento_unico();


create or replace function publicar.establecer_estado_pendiente_medicamento()
returns trigger as $$
begin
    if new.estado is null then
        new.estado := 'pendiente';
    end if;
    return new;
end;
$$ language plpgsql;

create trigger trg_establecer_estado_pendiente_medicamento
before insert on public.medicamentos
for each row execute function public.establecer_estado_pendiente_medicamento();

--cursores

create or replace function public.obtener_medicamentos_pendientes(historia_clinica_id integer)
returns void as $$
declare
    medicamento_cursor cursor for 
        select m.id, m.nombre, m.estado
        from public.medicamentos m
        where m.historia_clinica_id = historia_clinica_id
        and m.estado = 'pendiente';
    medicamento_record record;
begin
    open medicamento_cursor;
    loop
        fetch medicamento_cursor into medicamento_record;
        exit when not found;
        raise notice 'Medicamento ID: %, Nombre: %, Estado: %', medicamento_record.id, medicamento_record.nombre, medicamento_record.estado;
    end loop;
    close medicamento_cursor;
end;
$$ language plpgsql;

