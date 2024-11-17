-examenes

create or replace procedure crear_examen(
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
end;
$$;

create or replace procedure eliminar_examen(p_id int)
language plpgsql
as $$
begin
    delete from examenes where id = p_id;
end;
$$;

create or replace function modificar_examen(
    p_id int,
    p_nombre varchar,
    p_costo decimal,
    p_cubre_seguro boolean,
    p_fecha_realizacion date,
    p_estado estado_examen,
    p_historia_clinica_id int
) returns table(
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
    update examenes
    set nombre = p_nombre,
        costo = p_costo,
        cubre_seguro = p_cubre_seguro,
        fecha_realizacion = p_fecha_realizacion,
        estado = p_estado,
        historia_clinica_id = p_historia_clinica_id
    where id = p_id;

    return query select * from examenes where id = p_id;
end;
$$;


