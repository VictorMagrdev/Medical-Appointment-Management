-- public.informes
create or replace procedure public.generar_informe_citas_mes_medico(
	p_medico_id int,
	p_mes int,
	p_anio int
)
declare 
	v_contenido_informe jsonb;
begin
	 select jsonb_agg(jsonb_build_object(
        'fecha', c.fecha,
        'hora', c.hora,
        'motivo', c.motivo,
        'estado', c.estado,
        'paciente', p.nombre
    ))
    into v_contenido_informe from citas c
    join pacientes p on c.paciente_id = p.id
    where c.medico_id = p_medico_id
    and extract(month from c.fecha) = p_mes
    and extract(year from c.fecha) = p_anio;
     
   insert into public.informes(fecha, tipo_informe, contenido)
   values (current_date, 'informe de citas', v_contenido_informe);
     
end;
$$ language plpgsql;


create or replace procedure public.generar_informe_citas_pendientes_pacientes()
declare 
	v_contenido_informe jsonb;
begin
	select jsonb_agg(jsonb_build_object(
            'paciente', p.nombre,
            'citas_pendientes', jsonb_agg(
                jsonb_build_object(
                    'fecha', c.fecha,
                    'hora', c.hora,
                    'medico', m.nombre
                	)
            	)
        	))
    into v_contenido_informe from citas c
    join pacientes p on c.paciente_id = p.id
    join medicos m on c.medico_id = m.id
    where c.estado = 'programada'
    group by p.id, p.nombre;

    insert into informes(fecha, tipo_informe, contenido)
    values (current_date, 'informe de citas', v_contenido_informe);
end;
$$ language plpgsql;


create or replace procedure public.generar_informe_medicamentos_entregados()
declare 
	v_contenido_informe jsonb;
begin
	select 
        jsonb_agg(jsonb_build_object(
                'paciente', p.nombre,
                'medicamentos', jsonb_agg(
                    jsonb_build_object(
                        'medicamento', m.nombre,
                        'forma', m.forma_farmaceutica,
                        'dosis', m.dosis,
                        'indicaciones', m.indicaciones_uso
                    )
                )
            ))
    into v_contenido_informe
    from pacientes p
    join citas c on c.paciente_id = p.id
    join historias_clinicas hc on hc.cita_id = c.id
    join medicamentos m on m.historia_clinica_id = hc.id
    where m.estado = 'entregado'
    group by p.id, p.nombre;

    insert into informes (fecha, tipo_informe, contenido)
    values (current_date, 'informe de citas', medicamentos_informe);
end;
$$ language plpgsql;


create or replace procedure public.generar_informe_examenes_pendientes_pacientes()
declare 
	v_contenido_informe jsonb;
begin
	select 
        jsonb_agg(jsonb_build_object(
                'paciente', p.nombre,
                'examenes_pendientes', jsonb_agg(
                    jsonb_build_object(
                        'examen', e.nombre,
                        'costo', e.costo,
                        'fecha_realizacion', e.fecha_realizacion
                    )
                )
            ))
    into v_contenido_informe
    from pacientes p
    join citas c on c.paciente_id = p.id
    join historias_clinicas hc on hc.cita_id = c.id
    join examenes e on e.historia_clinica_id = hc.id
    where e.estado = 'pendiente'
    group by p.id, p.nombre;

    insert into informes (fecha, tipo_informe, contenido)
    values (current_date, 'examenes', v_contenido_informe);
end;
$$ language plpgsql;



