--public.pacientes

// CREAR PACIENTE
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

exception
	when unique_violation then
		rollback;
		raise notice 'La identificacion ya existe en el sistema.';
	
	when foreign_key_violation then
		rollback;
		raise notice 'El seguro asociado no existe.';

	when date_out_of_range then
        rollback;
        raise notice 'La fecha de nacimiento está fuera de un rango permitido.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
	
end;
$$;

// ELIMINAR PACIENTE
create or replace procedure public.eliminar_paciente(p_id int)
language plpgsql
as $$
begin
    delete from public.pacientes where id = p_id;
	
	if not found then
		raise exception 'Error: El paciente con ID % no existe', p_id;
	end if;
	
exception
	when others then
        raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;


// MODIFICAR PACIENTE 
create or replace procedure public.modificar_paciente(
    p_id int, p_nombre varchar,
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

	if not found then
	    raise exception 'El paciente no existe';
	end if;

exception 
	when unique_violation then
		rollback;
		raise notice 'La identificacion ya existe en el sistema.';
	
	when foreign_key_violation then
		rollback;
		raise notice 'El seguro asociado no existe.';

	when date_out_of_range then
        rollback;
        raise notice 'La fecha de nacimiento está fuera de un rango permitido.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

// OBTENER PACIENTES
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
	if not exists (select 1 from public.pacientes) then
        raise exception 'No se encontraron registros en la tabla de pacientes.';
    end if;	

    return query select * from public.pacientes;

exception
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

