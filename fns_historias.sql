-- CREAR HISTORIA CLÍNICA
create or replace procedure crear_historia_clinica(
    p_datos jsonb,
    p_cita_id int
)
language plpgsql
as $$
begin
    insert into historias_clinicas (datos, cita_id)
    values (p_datos, p_cita_id);

exception
    when foreign_key_violation then
        rollback;
        raise notice 'Error: La cita asociada no existe.';
    
    when null_value_not_allowed then
        rollback;
        raise notice 'Error: Uno de los valores obligatorios es NULL.';

    when others then
        rollback;
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- ELIMINAR HISTORIA CLÍNICA
create or replace procedure eliminar_historia_clinica(p_id int)
language plpgsql
as $$
begin
    -- Intentar eliminar la historia clínica
    delete from historias_clinicas where id = p_id;

    -- Verificar si no se encontró la historia clínica
    if not found then
        raise exception 'Error: La historia clínica con ID % no existe.', p_id;
    end if;

exception
    when others then
        rollback;
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- MODIFICAR HISTORIA CLÍNICA
create or replace function modificar_historia_clinica(
    p_id int,
    p_datos jsonb,
    p_cita_id int
) returns table(
    id int,
    datos jsonb,
    cita_id int
)
language plpgsql
as $$
begin
    -- Intentar actualizar la historia clínica
    update historias_clinicas
    set datos = p_datos,
        cita_id = p_cita_id
    where id = p_id;

    -- Verificar si no se encontró la historia clínica
    if not found then
        raise exception 'Error: La historia clínica con ID % no existe.', p_id;
    end if;

    return query select * from historias_clinicas where id = p_id;

exception
    when foreign_key_violation then
        rollback;
        raise notice 'Error: La cita asociada no existe.';
    
    when null_value_not_allowed then
        rollback;
        raise notice 'Error: Uno de los valores obligatorios es NULL.';
    
    when others then
        rollback;
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- OBTENER HISTORIAS CLÍNICAS
create or replace function obtener_historias_clinicas()
returns table(
    id int,
    datos jsonb,
    cita_id int
)
language plpgsql
as $$
begin
    -- Verificar si hay registros en la tabla
    if not exists (select 1 from historias_clinicas) then
        raise exception 'No se encontraron registros en la tabla de historias clínicas.';
    end if;

    return query select * from historias_clinicas;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;
