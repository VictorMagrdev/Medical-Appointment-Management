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
		raise notice 'Error: El paciente con ID % no existe', p_id;
		return;
	else
		raise notice 'El paciente con ID % ha sido eliminado correctamente', p_id;
	end if;
	
	
exception
	when others then
        raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;


// MODIFICAR PACIENTE 
create or replace function public.modificar_paciente(
    p_id int, p_nombre varchar,
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
	
	if not exists (select 1 from public.pacientes where id = p_id) then 
		raise notice 'El paciente no existe';
		return;
	end if;
	
	if not exists (select 1 from public.seguro_medico where id = p_seguro_id) then
		raise notice 'El seguro medico no existe';
		return;
	end if;

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
    return query select * from public.pacientes;
	
	if not found then
        raise notice 'No se encontraron registros en la tabla de pacientes.';
    end if;	

exception
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

