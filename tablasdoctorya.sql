create type tipo_seguro as enum ('publico', 'privado');
create type sexo as enum ('masculino', 'femenino', 'otro');
create type estado_cita as enum ('programada', 'cancelada', 'completada');
create type forma_farmaceutica as enum ('tableta', 'ampolla', 'suspension');
create type estado_medicamento as enum ('pendiente', 'entregado');
create type tipo_informe as enum ('informe de citas', 'examenes');
create type estado_examen as enum ('pendiente', 'efectuado');

create table especialidades(
	id serial primary key,
	nombre varchar(65) not null unique
);

create table seguro_medico (
    id serial primary key,
    nombre varchar(255) not null,
    tipo tipo_seguro,
    fecha_inicio date not null,
    fecha_final date not null,
    celular_contacto varchar(15)
);

create table public.pacientes (
    id serial primary key,
    nombre varchar(255) not null,
    identificacion varchar(50) unique not null,
    fecha_nacimiento date not null,
    sexo sexo,
    direccion varchar(255),
    email varchar(255),
    celular varchar(15),
    seguro_id int references seguro_medico(id)
);

create table medicos (
    id serial primary key,
    nombre varchar(255) not null,
    identificacion varchar(50) unique not null,
    registro_medico varchar(50) unique not null,
    especialidad_id integer,
    email varchar(255),
    celular varchar(15),
    foreign key (especialidad_id) references especialidades(id)
);

create table calendario (
    fecha date not null,
    hora time not null,
    medico_id int references medicos(id),
    primary key (fecha, hora, medico_id)
);

create table citas (
    id serial primary key,
    fecha date not null,
    hora time not null,
    motivo varchar(255),
    estado estado_cita,
    paciente_id int references public.pacientes(id),
    medico_id int references medicos(id),
    foreign key (fecha, hora, medico_id) references calendario(fecha, hora, medico_id)
);

create table historias_clinicas (
    id serial primary key,
	datos jsonb, /*  fecha date not null, sintomas text, diagnostico text, tratamiento text, observaciones text,*/
    cita_id int references citas(id)
);

create table medicamentos (
    id serial primary key,
    nombre varchar(255) not null,
    principio_activo varchar(255),
    forma_farmaceutica forma_farmaceutica,
    dosis varchar(50),
    indicaciones_uso text,
    duracion_tratamiento varchar(50),
    estado estado_medicamento,
    historia_clinica_id int references historias_clinicas(id)
);

create table examenes (
    id serial primary key,
    nombre varchar(255) not null,
    costo decimal(10, 2),
    cubre_seguro boolean,
    fecha_realizacion date,
    estado estado_examen,
    historia_clinica_id int references historias_clinicas(id)
);

create table resultados_examenes (
    id serial primary key,
    diagnostico text,
    posible_tratamiento text,
    examen_id int references examenes(id),
    medico_id int references medicos(id)
);

create table remisiones_medicas (
    id serial primary key,
    fecha date not null,
    motivo_remision text,
    medico_id int references medicos(id),
    historia_clinica_id int references historias_clinicas(id)
);

create table informes (
    id serial primary key,
    fecha date not null,
    tipo_informe tipo_informe,
    contenido json
);

create table documento_auditoria (
    id serial primary key,
    fecha date not null,
    nombre_paciente varchar(255) not null,
    nombre_doctor varchar(255) not null,
    motivo_cita text,
    diagnostico text,
    medicamentos_recetados text
);

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

--seguros medicos

create or replace procedure crear_seguro_medico(
    p_nombre varchar,
    p_tipo tipo_seguro,
    p_fecha_inicio date,
    p_fecha_final date,
    p_celular_contacto varchar
)
language plpgsql
as $$
begin
    insert into seguro_medico (nombre, tipo, fecha_inicio, fecha_final, celular_contacto)
    values (p_nombre, p_tipo, p_fecha_inicio, p_fecha_final, p_celular_contacto);
end;
$$;

create or replace procedure eliminar_seguro_medico(p_id int)
language plpgsql
as $$
begin
    delete from seguro_medico where id = p_id;
end;
$$;

create or replace function modificar_seguro_medico(
    p_id int,
    p_nombre varchar,
    p_tipo tipo_seguro,
    p_fecha_inicio date,
    p_fecha_final date,
    p_celular_contacto varchar
) returns table(
    id int,
    nombre varchar,
    tipo tipo_seguro,
    fecha_inicio date,
    fecha_final date,
    celular_contacto varchar
)
language plpgsql
as $$
begin
    update seguro_medico
    set nombre = p_nombre,
        tipo = p_tipo,
        fecha_inicio = p_fecha_inicio,
        fecha_final = p_fecha_final,
        celular_contacto = p_celular_contacto
    where id = p_id;

    return query select * from seguro_medico where id = p_id;
end;
$$;

create or replace function obtener_seguros_medicos()
returns table(
    id int,
    nombre varchar,
    tipo tipo_seguro,
    fecha_inicio date,
    fecha_final date,
    celular_contacto varchar
)
language plpgsql
as $$
begin
    return query select * from seguro_medico;
end;
$$;

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

--medicamentos

create or replace procedure crear_medicamento(
    p_nombre varchar,
    p_principio_activo varchar,
    p_forma_farmaceutica forma_farmaceutica,
    p_dosis varchar,
    p_indicaciones_uso text,
    p_duracion_tratamiento varchar,
    p_estado estado_medicamento,
    p_historia_clinica_id int
)
language plpgsql
as $$
begin
    insert into medicamentos (nombre, principio_activo, forma_farmaceutica, dosis, indicaciones_uso, duracion_tratamiento, estado, historia_clinica_id)
    values (p_nombre, p_principio_activo, p_forma_farmaceutica, p_dosis, p_indicaciones_uso, p_duracion_tratamiento, p_estado, p_historia_clinica_id);
end;
$$;

create or replace procedure eliminar_medicamento(p_id int)
language plpgsql
as $$
begin
    delete from medicamentos where id = p_id;
end;
$$;

create or replace function modificar_medicamento(
    p_id int,
    p_nombre varchar,
    p_principio_activo varchar,
    p_forma_farmaceutica forma_farmaceutica,
    p_dosis varchar,
    p_indicaciones_uso text,
    p_duracion_tratamiento varchar,
    p_estado estado_medicamento,
    p_historia_clinica_id int
) returns table(
    id int,
    nombre varchar,
    principio_activo varchar,
    forma_farmaceutica forma_farmaceutica,
    dosis varchar,
    indicaciones_uso text,
    duracion_tratamiento varchar,
    estado estado_medicamento,
    historia_clinica_id int
)
language plpgsql
as $$
begin
    update medicamentos
    set nombre = p_nombre,
        principio_activo = p_principio_activo,
        forma_farmaceutica = p_forma_farmaceutica,
        dosis = p_dosis,
        indicaciones_uso = p_indicaciones_uso,
        duracion_tratamiento = p_duracion_tratamiento,
        estado = p_estado,
        historia_clinica_id = p_historia_clinica_id
    where id = p_id;

    return query select * from medicamentos where id = p_id;
end;
$$;

create or replace function obtener_medicamentos()
returns table(
    id int,
    nombre varchar,
    principio_activo varchar,
    forma_farmaceutica forma_farmaceutica,
    dosis varchar,
    indicaciones_uso text,
    duracion_tratamiento varchar,
    estado estado_medicamento,
    historia_clinica_id int
)
language plpgsql
as $$
begin
    return query select * from medicamentos;
end;
$$;

--examenes

create or replace procedure crear_examen(
    p_nombre varchar,
    p_costo decimal,
    p_cubre_seguro boolean,
    p_fecha_realizacion date,
    p_estado estado_examen,
    p_historia_clinica_id int
)
language plpgsql
as $$
begin
    insert into examenes (nombre, costo, cubre_seguro, fecha_realizacion, estado, historia_clinica_id)
    values (p_nombre, p_costo, p_cubre_seguro, p_fecha_realizacion, p_estado, p_historia_clinica_id);
end;
$$;

create or replace procedure eliminar_examen(p_id int)
language plpgsql
as $$
begin
    delete from examenes where id = p_id;
end;
$$;

create or replace function modificar_examen(
    p_id int,
    p_nombre varchar,
    p_costo decimal,
    p_cubre_seguro boolean,
    p_fecha_realizacion date,
    p_estado estado_examen,
    p_historia_clinica_id int
) returns table(
    id int,
    nombre varchar,
    costo decimal,
    cubre_seguro boolean,
    fecha_realizacion date,
    estado estado_examen,
    historia_clinica_id int
)
language plpgsql
as $$
begin
    update examenes
    set nombre = p_nombre,
        costo = p_costo,
        cubre_seguro = p_cubre_seguro,
        fecha_realizacion = p_fecha_realizacion,
        estado = p_estado,
        historia_clinica_id = p_historia_clinica_id
    where id = p_id;

    return query select * from examenes where id = p_id;
end;
$$;


