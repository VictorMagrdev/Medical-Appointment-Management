--remisiones

--'<motivo>Urgente</motivo>'


create or replace function public.crear_remision_medica(
    p_fecha date,
    p_motivo_remision xml,
    p_medico_id int,
    p_historia_clinica_id int
)
returns void as $$
begin
    insert into public.remisiones_medicas(fecha, motivo_remision, medico_id, historia_clinica_id)
    values (p_fecha, p_motivo_remision, p_medico_id, p_historia_clinica_id);
end;
$$ language plpgsql;


create or replace function public.obtener_remision_medica(
    p_remision_id int
)
returns table (
    id int,
    fecha date,
    motivo_remision xml,
    medico_id int,
    historia_clinica_id int
) as $$
begin
    return query
    select id, fecha, motivo_remision, medico_id, historia_clinica_id
    from public.remisiones_medicas
    where id = p_remision_id;
end;
$$ language plpgsql;


create or replace function public.actualizar_remision_medica(
    p_remision_id int,
    p_fecha date,
    p_motivo_remision xml,
    p_medico_id int,
    p_historia_clinica_id int
)
returns void as $$
begin
    update public.remisiones_medicas
    set fecha = p_fecha,
        motivo_remision = p_motivo_remision,
        medico_id = p_medico_id,
        historia_clinica_id = p_historia_clinica_id
    where id = p_remision_id;
end;
$$ language plpgsql;


create or replace function public.eliminar_remision_medica(
    p_remision_id int
)
returns void as $$
begin
    delete from public.remisiones_medicas
    where id = p_remision_id;
end;
$$ language plpgsql;


create or replace function public.obtener_remisiones_medicas()
returns table (
    v_id int,
    v_fecha date,
    v_motivo_remision xml,
    v_medico_id int,
    v_historia_clinica_id int
) as $$
begin
    return query
    select id, fecha, motivo_remision, medico_id, historia_clinica_id
    from public.remisiones_medicas;
end;
$$ language plpgsql;


--CRUD NODOS XML

create or replace function public.agregar_nodo_xml(
    p_remision_id int,
    p_nodo_nombre text,
    p_nodo_valor text
)
returns void as $$
begin
    update public.remisiones_medicas
    set motivo_remision = 
        xmlappend(
            motivo_remision,
            xmlelement(name p_nodo_nombre, p_nodo_valor)
        )
    where id = p_remision_id;
end;
$$ language plpgsql;


create or replace function public.modificar_nodo_xml(
    p_remision_id int,
    p_nodo_nombre text,
    p_nuevo_valor text
)
returns void as $$
begin
    update public.remisiones_medicas
    set motivo_remision = 
        xmlreplace(
            motivo_remision,
            xpath('/root/' || p_nodo_nombre, motivo_remision),
            xmlelement(name p_nodo_nombre, p_nuevo_valor)
        )
    where id = p_remision_id;
end;
$$ language plpgsql;


create or replace function public.eliminar_nodo_xml(
    p_remision_id int,
    p_nodo_nombre text
)
returns void as $$
begin
    update public.remisiones_medicas
    set motivo_remision = 
        xmlremove(
            motivo_remision,
            xpath('/root/' || p_nodo_nombre, motivo_remision)
        )
    where id = p_remision_id;
end;
$$ language plpgsql;
