--historias clinicas

create or replace procedure crear_historia_clinica(
    p_datos jsonb,
    p_cita_id int
)
language plpgsql
as $$
begin
    insert into historias_clinicas (datos, cita_id)
    values (p_datos, p_cita_id);
end;
$$;

create or replace procedure eliminar_historia_clinica(p_id int)
language plpgsql
as $$
begin
    delete from historias_clinicas where id = p_id;
end;
$$;

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
    update historias_clinicas
    set datos = p_datos,
        cita_id = p_cita_id
    where id = p_id;

    return query select * from historias_clinicas where id = p_id;
end;
$$;

create or replace function obtener_historias_clinicas()
returns table(
    id int,
    datos jsonb,
    cita_id int
)
language plpgsql
as $$
begin
    return query select * from historias_clinicas;
end;
$$;

