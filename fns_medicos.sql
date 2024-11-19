--medicos

// CREAR MEDICO
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

exception
	when unique_violation then
		rollback;
		raise notice 'La identificacion ya existe en el sistema.';
	
	when foreign_key_violation then
		rollback;
		raise notice 'La especialidad asociada no existe.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;


// ELIMINAR MEDICO
create or replace procedure eliminar_medico(p_id int)
language plpgsql
as $$
begin
    delete from medicos where id = p_id;
	if not found then
		raise exception 'Error: El medico con ID % no existe', p_id;
	end if;
exception
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

// MODIFICAR MEDICO
create or replace procedure modificar_medico(
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

	if not found then
		raise exception 'Error: El medico con ID % no existe', p_id;
	end if;
exception
	when unique_violation then
		rollback;
		raise notice 'La identificacion ya existe en el sistema.';
	
	when foreign_key_violation then
		rollback;
		raise notice 'La especialidad asociada no existe.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

// OBTENER MEDICOS
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


