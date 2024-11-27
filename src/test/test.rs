
    use reqwest::Client;
    use serde_json::json;
    use tokio;

    const BASE_URL: &str = "http://127.0.0.1:8080/api/v1";
    //especialidad
    #[tokio::test]
    async fn test_post_especialidad() {
        let client = Client::new();
        let url = format!("{}/especialidades", BASE_URL);

        let json_body = json!({
            "nombre": "Cardiología"
        });

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        println!("{}", response.status());
        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn test_get_especialidad() {
        let client = Client::new();
        let url = format!("{}/especialidades", BASE_URL);

        let response = client.get(&url).send().await.expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_put_especialidad() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/especialidades/{}", BASE_URL, id);

        let json_body = json!({
            "nombre": "radiologia"
        }
        );

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //seguro medico
    #[tokio::test]
    async fn test_post_seguro_medico() {
        let client = Client::new();
        let url = format!("{}/seguros-medicos", BASE_URL);

        let json_body = json!({
          "nombre": "Seguro Salud Plus",
          "tipo_seguro": "privado",
          "fecha_inicio": "2024-01-01",
          "fecha_final": "2025-01-01",
            "estado":"activo",
          "celular_contacto": "+573001234567"
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn test_get_seguro_medico() {
        let client = Client::new();
        let url = format!("{}/seguros-medicos", BASE_URL);

        let response = client.get(&url).send().await.expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_put_seguro_medico() {
        let client = Client::new();
        let id = 1000;
        let url = format!("{}/seguros-medicos/{}", BASE_URL, id);

        let json_body = json!({
          "nombre": "Seguro Salud +",
          "tipo_seguro": "privado",
          "fecha_inicio": "2024-01-01",
          "fecha_final": "2025-01-01",
            "estado":"activo",
          "celular_contacto": "+573001234567"
        }
        );

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //cita
    #[tokio::test]
    async fn test_post_cita() {
        let client = Client::new();
        let url = format!("{}/cita", BASE_URL);

        let json_body = json!({
          "fecha": "2024-11-22",
          "hora": "10:30:00",
          "motivo": "Control de salud",
          "estado": "programada",
          "paciente_id": 6,
          "medico_id": 1
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 201);
    }

    #[tokio::test]
    async fn test_put_cita() {
        let client = Client::new();
        let id = 101;
        let url = format!("{}/cita/{}", BASE_URL, id);

        let json_body = json!({
          "estado": "completada"
        }
        );

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 200);
    }
    //historia clinica
    #[tokio::test]
    async fn test_post_historia_clinica() {
        let client = Client::new();
        let url = format!("{}/historias-clinicas", BASE_URL);

        let json_body = json!({
          "datos": {
            "peso": "75kg",
            "presion_arterial": "120/80"
          },
          "cita_id": 101
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 201);
    }

    #[tokio::test]
    async fn test_get_historia_clinica() {
        let client = Client::new();
        let url = format!("{}/historias-clinicas", BASE_URL);

        let response = client.get(&url).send().await.expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn test_put_historia_clinica() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/historias-clinicas/{}", BASE_URL, id);

        let json_body = json!({
          "fecha": "2024-11-20",
          "nombre_paciente": "Carlos Gómez",
          "nombre_doctor": "Dra. Ana Martínez",
          "motivo_cita": "Chequeo general",
          "diagnostico": "Presión arterial alta",
          "medicamentos_recetados": "Losartán 50 mg, Aspirina 81 mg"
        }
        );

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //medicamento
    #[tokio::test]
    async fn test_post_medicamento() {
        let client = Client::new();
        let url = format!("{}/medicamentos", BASE_URL);

        let json_body = json!({
          "nombre": "Amoxicilina",
          "principio_activo": "Amoxicilina",
          "forma_farmaceutica": "tableta",
          "dosis": "500mg",
          "indicaciones_uso": "Tomar con agua cada 8 horas",
          "duracion_tratamiento": "7 días",
          "estado": "pendiente",
          "historia_clinica_id": 29
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 201);
    }

    #[tokio::test]
    async fn test_get_medicamentos() {
        let client = Client::new();
        let url = format!("{}/medicamentos", BASE_URL);

        let response = client.get(&url).send().await.expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn test_put_medicamento() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/medicamentos/{}", BASE_URL, id);

        let json_body = json!({
          "fecha": "2024-11-20",
          "nombre_paciente": "Carlos Gómez",
          "nombre_doctor": "Dra. Ana Martínez",
          "motivo_cita": "Chequeo general",
          "diagnostico": "Presión arterial alta",
          "medicamentos_recetados": "Losartán 50 mg, Aspirina 81 mg"
        }
        );

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //auditoria
    #[tokio::test]
    async fn test_get_auditoria_by_id() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/auditorias/{}", BASE_URL, id);

        let response = client.get(&url).send().await.expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_put_auditoria_by_id() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/auditorias/{}", BASE_URL, id);

        let json_body = json!({
          "fecha": "2024-11-20",
          "nombre_paciente": "Carlos Gómez",
          "nombre_doctor": "Dra. Ana Martínez",
          "motivo_cita": "Chequeo general",
          "diagnostico": "Presión arterial alta",
          "medicamentos_recetados": "Losartán 50 mg, Aspirina 81 mg"
        }
        );

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //paciente
    #[tokio::test]
    async fn test_post_paciente() {
        let client = Client::new();
        let url = format!("{}/pacientes", BASE_URL);

        let json_body = json!({
          "nombre": "María López",
          "identificacion": "987654321",
          "fecha_nacimiento": "1990-05-15",
          "sexo": "femenino",
          "direccion": "Calle 123, Ciudad Ejemplo",
          "email": "maria.lopez@ejemplo.com",
          "celular": "+573002345678",
          "seguro_id": 1000
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn test_put_paciente() {
        let client = Client::new();
        let id = 6;
        let url = format!("{}/pacientes/{}", BASE_URL, id);

        let json_body = json!({
          "nombre": "bibiana López",
          "identificacion": "987654321",
          "fecha_nacimiento": "1990-05-15",
          "sexo": "femenino",
          "direccion": "Calle 123, Ciudad Ejemplo",
          "email": "maria.lopez@ejemplo.com",
          "celular": "+573002345678",
          "seguro_id": 1000
        });

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //examen
    #[tokio::test]
    async fn test_post_examen() {
        let client = Client::new();
        let url = format!("{}/examenes", BASE_URL);

        let json_body = json!({
          "nombre": "Hemograma completo",
          "costo": 45.50,
          "cubre_seguro": true,
          "fecha_realizacion": "2024-11-22",
          "estado": "pendiente",
          "historia_clinica_id": 29
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 201);
    }

    #[tokio::test]
    async fn test_put_examen() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/examenes/{}", BASE_URL, id);

        let json_body = json!({
          "nombre": "encefalograma completo",
          "costo": 45.50,
          "cubre_seguro": true,
          "fecha_realizacion": "2024-11-22",
          "estado": "pendiente",
          "historia_clinica_id": 123
        });

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //medico
    #[tokio::test]
    async fn test_post_medico() {
        let client = Client::new();
        let url = format!("{}/medicos", BASE_URL);

        let json_body = json!({
          "nombre": "Juan Pérez",
          "identificacion": "123456789",
          "registro_medico": "RM12345",
          "especialidad_id": 1,
          "email": "juan.perez@ejemplo.com",
          "celular": "+573001234567"
        }
        );

        let response = client
            .post(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn test_put_medico() {
        let client = Client::new();
        let id = 3000;
        let url = format!("{}/medicos/{}", BASE_URL, id);

        let json_body = json!({
          "nombre": "roberto Pérez",
          "identificacion": "123456789",
          "registro_medico": "RM12345",
          "especialidad_id": 1,
          "email": "juan.perez@ejemplo.com",
          "celular": "+573001234567"
        });

        let response = client
            .put(&url)
            .json(&json_body)
            .send()
            .await
            .expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 200);
    }

    //calendario
    #[tokio::test]
    async fn test_get_calendario_por_medico() {
        let client = Client::new();
        let id =3000;
        let url = format!("{}/calendario/medico/{}", BASE_URL, id);

        let response = client.get(&url).send().await.expect("Error en la petición");
        let status = response.status();
        let body = response.text().await.expect("Error al leer el cuerpo de la respuesta");

        println!("body del request = {:?}", body);

        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn get_calendario_por_especialidad() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/calendario/especialidad/{}", BASE_URL, id);

        let response = client.get(&url).send().await.expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn get_calendario_por_paciente() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/calendario/paciente/{}", BASE_URL, id);

        let response = client.get(&url).send().await.expect("Error en la petición");
        assert_eq!(response.status(), 200);
    }

    //delete
    #[tokio::test]
    async fn test_delete_especialidad() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/especialidades/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_seguro_medico() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/seguros-medicos/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_examen() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/examenes/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_medico() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/medicos/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_paciente() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/pacientes/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_auditoria() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/auditorias/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_medicamento() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/medicamentos/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }

    #[tokio::test]
    async fn test_delete_historia_clinica() {
        let client = Client::new();
        let id = 1;
        let url = format!("{}/historias-clinicas/{}", BASE_URL, id);

        let response = client
            .delete(&url)
            .send()
            .await
            .expect("Error en la petición");
        assert_eq!(response.status(), 204);
    }
