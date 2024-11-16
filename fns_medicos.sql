--medicos

create or replace procedure crear_medico(
    p_nombre varchar,
    p_identificacion varchar,
    p_registro_medico varchar,
    p_especialidad_id int,
    p_email varchar,
    p_celular varchar
)
language plpgsql
as $$
begin
    insert into medicos (nombre, identificacion, registro_medico, especialidad_id, email, celular)
    values (p_nombre, p_identificacion, p_registro_medico, p_especialidad_id, p_email, p_celular);
end;
$$;

create or replace procedure eliminar_medico(p_id int)
language plpgsql
as $$
begin
    delete from medicos where id = p_id;
end;
$$;

create or replace function modificar_medico(
    p_id int,
    p_nombre varchar,
    p_identificacion varchar,
    p_registro_medico varchar,
    p_especialidad_id int,
    p_email varchar,
    p_celular varchar
) returns table(
    id int,
    nombre varchar,
    identificacion varchar,
    registro_medico varchar,
    especialidad_id int,
    email varchar,
    celular varchar
)
language plpgsql
as $$
begin
    update medicos
    set nombre = p_nombre,
        identificacion = p_identificacion,
        registro_medico = p_registro_medico,
        especialidad_id = p_especialidad_id,
        email = p_email,
        celular = p_celular
    where id = p_id;

    return query select * from medicos where id = p_id;
end;
$$;

create or replace function obtener_medicos()
returns table(
    id int,
    nombre varchar,
    identificacion varchar,
    registro_medico varchar,
    especialidad_id int,
    email varchar,
    celular varchar
)
language plpgsql
as $$
begin
    return query select * from medicos;
end;
$$;

