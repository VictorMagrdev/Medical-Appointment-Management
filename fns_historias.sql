-- CREAR HISTORIA CLÍNICA
create or replace procedure crear_historia_clinica(
    p_datos xml,
    p_cita_id bigint
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
    delete from historias_clinicas where id = p_id;

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
create or replace procedure modificar_historia_clinica(
    p_id int,
    p_datos xml,
    p_cita_id int
)
language plpgsql
as $$
begin
    update historias_clinicas
    set datos = p_datos,
        cita_id = p_cita_id
    where id = p_id;

    if not found then
        raise exception 'Error: La historia clínica con ID % no existe.', p_id;
    end if;

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

-- OBTENER HISTORIAS CLINICAS
create or replace function obtener_historias_clinicas()
returns table(
    id bigint,
    datos xmlx1,
    cita_id int
)
language plpgsql
as $$
begin
    if not exists (select 1 from historias_clinicas) then
        raise exception 'No se encontraron registros en la tabla de historias clínicas.';
    end if;

    return query select * from historias_clinicas;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;
