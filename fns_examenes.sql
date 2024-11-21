

-- Crear Examen
create or replace procedure public.crear_examen(
    p_nombre varchar,
    p_costo decimal,
    p_cubre_seguro boolean,
    p_fecha_realizacion date,
    p_estado estado_examen,
    p_historia_clinica_id int
)
language plpgsql
as $$
begin
    insert into examenes (nombre, costo, cubre_seguro, fecha_realizacion, estado, historia_clinica_id)
    values (p_nombre, p_costo, p_cubre_seguro, p_fecha_realizacion, p_estado, p_historia_clinica_id);

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

-- Eliminar Examen
create or replace procedure public.eliminar_examen(p_id int)
language plpgsql
as $$
begin
    delete from examenes where id = p_id;

    if not found then
        raise exception 'Error: El examen con ID % no existe.', p_id;
    end if;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- Modificar Examen
create or replace procedure public.modificar_examen(
    p_id int,
    p_nombre varchar,
    p_costo decimal,
    p_cubre_seguro boolean,
    p_fecha_realizacion date,
    p_estado estado_examen,
    p_historia_clinica_id int
)
language plpgsql
as $$
begin
    update examenes
    set nombre = p_nombre,
        costo = p_costo,
        cubre_seguro = p_cubre_seguro,
        fecha_realizacion = p_fecha_realizacion,
        estado = p_estado,
        historia_clinica_id = p_historia_clinica_id
    where id = p_id;

    if not found then
        raise exception 'Error: El examen con ID % no existe.', p_id;
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

-- Obtener Exámenes
create or replace function public.obtener_examenes()
returns table(
    id int,
    nombre varchar,
    costo decimal,
    cubre_seguro boolean,
    fecha_realizacion date,
    estado estado_examen,
    historia_clinica_id int
)
language plpgsql
as $$
begin
    if not exists (select 1 from examenes) then
        raise exception 'No se encontraron registros en la tabla de exámenes.';
    end if;

    return query select * from examenes;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;













