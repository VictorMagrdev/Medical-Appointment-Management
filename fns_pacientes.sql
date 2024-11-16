--public.pacientes

create or replace procedure public.crear_paciente(
    p_nombre varchar,
    p_identificacion varchar,
    p_fecha_nacimiento date,
    p_sexo sexo,
    p_direccion varchar,
    p_email varchar,
    p_celular varchar,
    p_seguro_id int
)
language plpgsql
as $$
begin
    insert into public.pacientes (nombre, identificacion, fecha_nacimiento, sexo, direccion, email, celular, seguro_id)
    values (p_nombre, p_identificacion, p_fecha_nacimiento, p_sexo, p_direccion, p_email, p_celular, p_seguro_id);
end;
$$;

create or replace procedure public.eliminar_paciente(p_id int)
language plpgsql
as $$
begin
    delete from public.pacientes where id = p_id;
end;
$$;

create or replace function public.modificar_paciente(
    p_id int,
    p_nombre varchar,
    p_identificacion varchar,
    p_fecha_nacimiento date,
    p_sexo sexo,
    p_direccion varchar,
    p_email varchar,
    p_celular varchar,
    p_seguro_id int
) returns table(
    id int,
    nombre varchar,
    identificacion varchar,
    fecha_nacimiento date,
    sexo sexo,
    direccion varchar,
    email varchar,
    celular varchar,
    seguro_id int
)
language plpgsql
as $$
begin
    update public.pacientes
    set nombre = p_nombre,
        identificacion = p_identificacion,
        fecha_nacimiento = p_fecha_nacimiento,
        sexo = p_sexo,
        direccion = p_direccion,
        email = p_email,
        celular = p_celular,
        seguro_id = p_seguro_id
    where id = p_id;

    return query select * from public.pacientes where id = p_id;
end;
$$;

create or replace function public.obtener_pacientes()
returns table(
    id int,
    nombre varchar,
    identificacion varchar,
    fecha_nacimiento date,
    sexo sexo,
    direccion varchar,
    email varchar,
    celular varchar,
    seguro_id int
)
language plpgsql
as $$
begin
    return query select * from public.pacientes;
end;
$$;

