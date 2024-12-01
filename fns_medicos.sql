--medicos


create or replace procedure public.crear_medico(
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
  
    insert into public.medicos (nombre, identificacion, registro_medico, especialidad_id, email, celular)
    values (p_nombre, p_identificacion, p_registro_medico, p_especialidad_id, p_email, p_celular);

exception
	when unique_violation then
		raise notice 'La identificacion ya existe en el sistema.';
	
	when foreign_key_violation then
		raise notice 'La especialidad asociada no existe.';
	
	when null_value_not_allowed then
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;



create or replace procedure public.eliminar_medico(p_id int)
language plpgsql
as $$
begin
    delete from public.medicos where id = p_id;
	if not found then
		raise exception 'Error: El medico con ID % no existe', p_id;
	end if;
exception
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;


create or replace procedure public.modificar_medico(
    p_id int,
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

    update public.medicos
    set nombre = p_nombre,
        identificacion = p_identificacion,
        registro_medico = p_registro_medico,
        especialidad_id = p_especialidad_id,
        email = p_email,
        celular = p_celular
    where id = p_id;

	if not found then
		raise exception 'Error: El medico con ID % no existe', p_id;
	end if;
exception
	when unique_violation then
		raise notice 'La identificacion ya existe en el sistema.';
	
	when foreign_key_violation then
		raise notice 'La especialidad asociada no existe.';
	
	when null_value_not_allowed then
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;


create or replace function public.obtener_medicos()
returns table(
    v_id int,
    v_nombre varchar,
    v_identificacion varchar,
    v_registro_medico varchar,
    v_especialidad_id int,
    v_email varchar,
    v_celular varchar
)
language plpgsql
as $$
begin
	if not exists (select 1 from public.medicos) then
        raise exception 'No se encontraron registros en la tabla de pacientes.';
    end if;	

    return query select id, nombre, identificacion, registro_medico, especialidad_id, email, celular
    from medicos;

exception
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;

end;
$$;

create or replace function public.obtener_medico(
    p_medico_id int
)
returns varchar as $$
begin
    return (
        select *
        from public.medicos where id = p_medico_id
    );
end;
$$ language plpgsql;

create or replace function public.obtener_especialidad_medico(
    p_medico_id int
)
returns varchar as $$
begin
    return (
        select e.nombre
        from public.especialidades e
        join public.medicos m on m.especialidad_id = e.id
        where m.id = p_medico_id
    );
end;
$$ language plpgsql;


create or replace function public.obtener_identificacion_medico(
    p_medico_id int
)
returns varchar as $$
begin
    return (
        select identificacion from public.medicos
		where id = p_medico_id;
    );
end;
$$ language plpgsql;






