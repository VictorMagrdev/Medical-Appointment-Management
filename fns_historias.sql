-- CREAR HISTORIA CLÍNICA
create or replace procedure public.crear_historia_clinica(
    p_datos jsonb,
    p_cita_id bigint
)
language plpgsql
as $$
begin
insert into public.historias_clinicas (datos,f echa, cita_id)
values (p_datos, now(), p_cita_id);

exception
    when foreign_key_violation then
        raise exception 'Error: La cita asociada no existe.';

when null_value_not_allowed then
        raise exception 'Error: Uno de los valores obligatorios es NULL.';

when others then
        raise exception 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;


-- ELIMINAR HISTORIA CLÍNICA
create or replace procedure public.eliminar_historia_clinica(p_id int)
language plpgsql
as $$
begin
    delete from public.historias_clinicas where id = p_id;

    if not found then
        raise exception 'Error: La historia clínica con ID % no existe.', p_id;
    end if;

exception
    when others then
        rollback;
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;


create or replace procedure public.modificar_historia_clinica(
    p_id int,
    p_datos jsonb,
    p_cita_id int
)
language plpgsql
as $$
begin
    update public.historias_clinicas
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
create or replace function public.obtener_historias_clinicas()
returns table(
    v_id bigint,
    v_fecha date,
    v_datos jsonb,
    v_cita_id int
)
language plpgsql
as $$
begin
    if not exists (select 1 from public.historias_clinicas) then
        raise exception 'No se encontraron registros en la tabla de historias clínicas.';
    end if;

    return query select * from public.historias_clinicas;

exception
    when others then
        raise notice 'Error: Ocurrió un error inesperado: %', sqlerrm;
end;
$$;

-- funciones

create or replace function public.obtener_sintomas_historia(
    p_historia_id bigint
)
returns text as $$
begin
    return (
        select datos ->> 'sintomas'
        from public.historias_clinicas
        where id = p_historia_id
    );
end;
$$ language plpgsql;

create or replace function public.obtener_diagnostico_historia(
    p_historia_id bigint
)
returns text as $$
begin
    return (
        select datos ->> 'diagnostico'
        from public.historias_clinicas
        where id = p_historia_id
    );
end;
$$ language plpgsql;

create or replace function public.obtener_tratamiento_historia(
    p_historia_id bigint
)
returns text as $$
begin
    return (
        select datos ->> 'tratamiento'
        from public.historias_clinicas
        where id = p_historia_id
    );
end;
$$ language plpgsql;



--trigger

create or replace function public.actualizar_estado_cita()
returns trigger
language plpgsql
as $$
begin
    perform public.cambiar_estado_cita(NEW.cita_id, 'completada');
    return NEW;
end;
$$;

create trigger trigger_actualizar_estado_cita
after insert on public.historias_clinicas
for each row execute function public.actualizar_estado_cita();




