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
        rollback;
        raise notice 'La historia clínica asociada no existe.';
    
    when null_value_not_allowed then
        rollback;
        raise notice 'Uno de los valores obligatorios es NULL.';
    
    when others then
        rollback;
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
        rollback;
        raise notice 'La historia clínica asociada no existe.';
    
    when null_value_not_allowed then
        rollback;
        raise notice 'Uno de los valores obligatorios es NULL.';
    
    when others then
        rollback;
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- Obtener Medicamentos
create or replace function public.obtener_medicamentos()
returns table(
    id int,
    nombre varchar,
    principio_activo varchar,
    forma_farmaceutica varchar,
    dosis varchar,
    indicaciones_uso text,
    duracion_tratamiento varchar,
    estado varchar,
    historia_clinica_id int
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


