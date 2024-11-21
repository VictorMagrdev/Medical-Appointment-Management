--public.especialidades

// CREAR ESPECIALIDAD
create or replace procedure public.crear_especialidad(
    p_nombre varchar
)
language plpgsql
as $$
begin	

    insert into public.especialidades (nombre)
    values (p_nombre);

exception
	when unique_violation then
		rollback;
		raise notice 'El nombre de la especialidad ya existe en el sistema.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
	
end;
$$;

// ELIMINAR ESPECIALIDAD
create or replace procedure public.eliminar_especialidad(p_id int)
language plpgsql
as $$
begin
    delete from public.especialidades where id = p_id;
	
	if not found then
		raise exception 'Error: La especialidad con ID % no existe', p_id;
	end if;
	
exception
	when others then
        raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

// MODIFICAR ESPECIALIDAD 
create or replace procedure public.modificar_especialidad(
    p_id int, 
    p_nombre varchar
)
language plpgsql
as $$
begin

    update public.especialidades
    set nombre = p_nombre
    where id = p_id;

	if not found then
	    raise exception 'La especialidad no existe';
	end if;

exception 
	when unique_violation then
		rollback;
		raise notice 'El nombre de la especialidad ya existe en el sistema.';
	
	when null_value_not_allowed then
		rollback;
		raise notice 'Uno de los valores obligatorios es NULL';
	
	when others then
		rollback;
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;

// OBTENER ESPECIALIDADES
create or replace function public.obtener_especialidades()
returns table(
    id int,
    nombre varchar
)
language plpgsql
as $$
begin
	if not exists (select 1 from public.especialidades) then
        raise exception 'No se encontraron registros en la tabla de especialidades.';
    end if;	

    return query select * from public.especialidades;

exception
	when others then
		raise notice 'Error: Ocurrio un error inesperado: %', sqlerrm;
end;
$$;






