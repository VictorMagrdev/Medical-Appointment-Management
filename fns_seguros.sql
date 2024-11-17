--seguros medicos

create or replace procedure crear_seguro_medico(
    p_nombre varchar,
    p_tipo tipo_seguro,
    p_fecha_inicio date,
    p_fecha_final date,
    p_celular_contacto varchar
)
language plpgsql
as $$
begin
    insert into seguro_medico (nombre, tipo, fecha_inicio, fecha_final, celular_contacto)
    values (p_nombre, p_tipo, p_fecha_inicio, p_fecha_final, p_celular_contacto);
end;
$$;

create or replace procedure eliminar_seguro_medico(p_id int)
language plpgsql
as $$
begin
    delete from seguro_medico where id = p_id;
end;
$$;

create or replace function modificar_seguro_medico(
    p_id int,
    p_nombre varchar,
    p_tipo tipo_seguro,
    p_fecha_inicio date,
    p_fecha_final date,
    p_celular_contacto varchar
) returns table(
    id int,
    nombre varchar,
    tipo tipo_seguro,
    fecha_inicio date,
    fecha_final date,
    celular_contacto varchar
)
language plpgsql
as $$
begin
    update seguro_medico
    set nombre = p_nombre,
        tipo = p_tipo,
        fecha_inicio = p_fecha_inicio,
        fecha_final = p_fecha_final,
        celular_contacto = p_celular_contacto
    where id = p_id;

    return query select * from seguro_medico where id = p_id;
end;
$$;

create or replace function obtener_seguros_medicos()
returns table(
    id int,
    nombre varchar,
    tipo tipo_seguro,
    fecha_inicio date,
    fecha_final date,
    celular_contacto varchar
)
language plpgsql
as $$
begin
    return query select * from seguro_medico;
end;
$$;

