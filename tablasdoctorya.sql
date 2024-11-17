create type tipo_seguro as enum ('publico', 'privado');
create type sexo as enum ('masculino', 'femenino', 'otro');
create type estado_cita as enum ('programada', 'cancelada', 'completada');
create type forma_farmaceutica as enum ('tableta', 'ampolla', 'suspension');
create type estado_medicamento as enum ('pendiente', 'entregado');
create type tipo_informe as enum ('informe de citas', 'examenes');
create type estado_examen as enum ('pendiente', 'efectuado');

create table public.especialidades(
	id serial primary key,
 	nombre varchar(65) not null unique
);

create table public.seguro_medico (
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

create table public.medicos (
    id serial primary key,
    nombre varchar(255) not null,
    identificacion varchar(50) unique not null,
    registro_medico varchar(50) unique not null,
    especialidad_id integer,
    email varchar(255),
    celular varchar(15),
    foreign key (especialidad_id) references especialidades(id)
);

create table public.calendario (
    fecha date not null,
    hora time not null,
    medico_id int references medicos(id),
    primary key (fecha, hora, medico_id)
);

create table public.citas (
    id serial primary key,
    fecha date not null,
    hora time not null,
    motivo varchar(255),
    estado estado_cita,
    paciente_id int references public.pacientes(id),
    medico_id int references medicos(id),
    foreign key (fecha, hora, medico_id) references calendario(fecha, hora, medico_id)
);

create table public.historias_clinicas (
    id serial primary key,
	datos jsonb, /*  fecha date not null, sintomas text, diagnostico text, tratamiento text, observaciones text,*/
    cita_id int references citas(id)
);

create table public.medicamentos (
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

create table public.examenes (
    id serial primary key,
    nombre varchar(255) not null,
    costo decimal(10, 2),
    cubre_seguro boolean,
    fecha_realizacion date,
    estado estado_examen,
    historia_clinica_id int references historias_clinicas(id)
);

create table public.resultados_examenes (
    id serial primary key,
    diagnostico text,
    posible_tratamiento text,
    examen_id int references examenes(id),
    medico_id int references medicos(id)
);

create table public.remisiones_medicas (
    id serial primary key,
    fecha date not null,
    motivo_remision text,
    medico_id int references medicos(id),
    historia_clinica_id int references historias_clinicas(id)
);

create table public.informes (
    id serial primary key,
    fecha date not null,
    tipo_informe tipo_informe,
    contenido jsonb
);


