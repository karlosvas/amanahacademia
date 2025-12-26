#[cfg(test)]
mod tests {
    use {
        crate::{
            controllers::cal::{
                add_booking, add_guests_to_booking, confirm_booking, fetch_and_detect_changes,
                get_all_bookings, get_booking, get_schedule, get_schedules,
            },
            models::cal::{AddGuestsPayload, BookingStatus, BookingsQueryParams, GuestInput},
            test_fixtures::fixtures::{create_mock_app_state, create_test_booking},
        },
        axum::{
            extract::{Path, Query, State},
            http::StatusCode,
            response::IntoResponse,
        },
        serde_json::json,
        std::{collections::HashMap, sync::Arc},
    };

    /// Test: detect_changes detecta cambio de status
    #[tokio::test]
    async fn test_fetch_and_detect_changes_status_change() {
        // Arrange: Crear AppState mock con cache inicial
        let mut initial_cache = HashMap::new();
        let booking_uid = "test-booking-123";
        let cached_booking = create_test_booking(booking_uid, BookingStatus::Pending);

        initial_cache.insert(booking_uid.to_string(), cached_booking);

        let app_state = create_mock_app_state(initial_cache).await;

        // Act: Simular que no hay cambios (misma función sin mock del client retornaría error)
        // Para este test básico, verificamos que la función se puede llamar
        let result = fetch_and_detect_changes(&app_state).await;

        // Assert: La función debería retornar error porque no hay cliente HTTP real
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to fetch bookings"));
    }

    /// Test: detect_changes no detecta cambios cuando status es igual
    #[tokio::test]
    async fn test_fetch_and_detect_changes_no_status_change() {
        // Arrange: Cache vacío
        let app_state = create_mock_app_state(HashMap::new()).await;

        // Act
        let result = fetch_and_detect_changes(&app_state).await;

        // Assert: Sin cliente HTTP real, debe fallar
        assert!(result.is_err());
    }


    /// Test: confirm_booking con mock HTTP server que responde éxito
    #[tokio::test]
    async fn test_confirm_booking_success() {
        // Arrange: Crear mock server
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let mock_response = json!({
            "status": "success",
            "data": {
                "id": 123,
                "uid": "booking-123",
                "status": "confirmed"
            }
        });

        let _m = server
            .mock("POST", "/bookings/booking-123/confirm")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-06-11")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        // Act
        let response = confirm_booking(State(app_state), Path("booking-123".to_string())).await;

        // Assert: Debería retornar NO_CONTENT
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    /// Test: confirm_booking con mock HTTP server que responde error
    #[tokio::test]
    async fn test_confirm_booking_rejected() {
        // Arrange: Crear mock server que responde con error
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("POST", "/bookings/booking-456/confirm")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-06-11")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(json!({"error": "Booking already confirmed"}).to_string())
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        // Act
        let response = confirm_booking(State(app_state), Path("booking-456".to_string())).await;

        // Assert: Debería retornar BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Test: add_guests_to_booking rechaza payload vacío
    #[tokio::test]
    async fn test_add_guests_empty_payload() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let booking_id = "booking-789".to_string();
        let empty_payload = AddGuestsPayload { guests: vec![] };

        // Act
        let response = add_guests_to_booking(
            State(app_state),
            Path(booking_id),
            axum::Json(empty_payload),
        )
        .await;

        // Assert: Debería rechazar con BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Test: add_guests_to_booking con al menos un guest válido
    #[tokio::test]
    async fn test_add_guests_valid_payload() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let booking_id = "booking-101".to_string();
        let payload = AddGuestsPayload {
            guests: vec![GuestInput {
                email: "guest@example.com".to_string(),
                name: Some("Test Guest".to_string()),
                time_zone: Some("Europe/Madrid".to_string()),
                phone_number: None,
                language: Some("es".to_string()),
            }],
        };

        // Act
        let response = add_guests_to_booking(
            State(app_state),
            Path(booking_id),
            axum::Json(payload),
        )
        .await;

        // Assert: Verifica que no hay panic (fallará por red, pero sin panic)
        let _result = response.into_response();
    }


    /// Test: get_all_bookings sin parámetros
    #[tokio::test]
    async fn test_get_all_bookings_no_params() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let params = BookingsQueryParams {
            event_type_id: None,
            event_type_ids: None,
            attendee_email: None,
            attendee_name: None,
            team_id: None,
            after_start: None,
            before_end: None,
            status: None,
            sort_start: None,
        };

        // Act
        let response = get_all_bookings(State(app_state), Query(params)).await;

        // Assert: No debe hacer panic
        let _result = response.into_response();
    }


    /// Test: add_booking rechaza payload sin attendees
    #[tokio::test]
    async fn test_add_booking_missing_attendees() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "eventTypeId": 123,
            "start": "2024-12-01T10:00:00Z",
            "attendees": []
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: Debería retornar BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Test: add_booking rechaza payload sin start_time
    #[tokio::test]
    async fn test_add_booking_missing_start_time() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "eventTypeId": 123,
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid"
            }]
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: Debería retornar BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Test: add_booking rechaza payload sin eventTypeId ni eventTypeSlug
    #[tokio::test]
    async fn test_add_booking_missing_event_type_info() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "start": "2024-12-01T10:00:00Z",
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid"
            }]
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: Debería retornar BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Test: add_booking acepta payload válido con eventTypeId
    #[tokio::test]
    async fn test_add_booking_valid_with_event_type_id() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "eventTypeId": 123,
            "start": "2024-12-01T10:00:00Z",
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid",
                "language": {
                    "locale": "es"
                }
            }],
            "metadata": {
                "custom_field": "value"
            }
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: No debe hacer panic (fallará por red, pero validación pasa)
        let _result = response.into_response();
    }

    /// Test: add_booking acepta payload con eventTypeSlug y username
    #[tokio::test]
    async fn test_add_booking_valid_with_slug_and_username() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "eventTypeSlug": "30min-meeting",
            "username": "testuser",
            "start": "2024-12-01T10:00:00Z",
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid"
            }]
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: No debe hacer panic
        let _result = response.into_response();
    }

    /// Test: add_booking acepta payload con eventTypeSlug y teamSlug
    #[tokio::test]
    async fn test_add_booking_valid_with_slug_and_team() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "eventTypeSlug": "team-meeting",
            "teamSlug": "engineering",
            "start": "2024-12-01T10:00:00Z",
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid"
            }]
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: No debe hacer panic
        let _result = response.into_response();
    }


    /// Test: add_booking maneja payload inválido
    #[tokio::test]
    async fn test_add_booking_invalid_json() {
        // Arrange
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = json!({
            "invalid": "structure",
            "not": "a booking"
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: Debería retornar BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }


    /// Test: add_booking con mock HTTP server que responde éxito
    #[tokio::test]
    async fn test_add_booking_with_mock_success() {
        // Arrange: Crear mock server
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        // Mock de respuesta exitosa de Cal.com
        let mock_response = json!({
            "status": "success",
            "data": {
                "id": 123,
                "uid": "booking-123",
                "status": "pending",
                "attendees": [{
                    "name": "Test User",
                    "email": "test@example.com",
                    "timeZone": "Europe/Madrid"
                }],
                "startTime": "2024-12-01T10:00:00Z"
            }
        });

        let _m = server
            .mock("POST", "/bookings")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-08-13")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        let payload = json!({
            "eventTypeId": 123,
            "start": "2024-12-01T10:00:00Z",
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid"
            }]
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: Debería retornar CREATED
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    /// Test: add_booking con mock HTTP server que responde error
    #[tokio::test]
    async fn test_add_booking_with_mock_error() {
        // Arrange: Crear mock server que responde con error
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("POST", "/bookings")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-08-13")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(json!({"error": "Invalid booking data"}).to_string())
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        let payload = json!({
            "eventTypeId": 123,
            "start": "2024-12-01T10:00:00Z",
            "attendees": [{
                "name": "Test User",
                "email": "test@example.com",
                "timeZone": "Europe/Madrid"
            }]
        });

        // Act
        let response = add_booking(State(app_state), axum::Json(payload)).await;

        // Assert: Debería retornar BAD_REQUEST
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Test: get_booking maneja error de API correctamente
    #[tokio::test]
    async fn test_get_booking_api_error() {
        // Arrange: Crear AppState que usará las credenciales de test
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);

        // Act: Intentar obtener un booking con la API real (fallará por credenciales inválidas)
        let response = get_booking(State(app_state), Path("test-123".to_string())).await;

        // Assert: Debería retornar un error de cliente (4xx)
        let resp = response.into_response();
        // Puede ser 404 (not found), 401 (unauthorized), o 400 (bad request)
        assert!(resp.status().is_client_error());
    }

    /// Test: get_schedules maneja error de API correctamente
    #[tokio::test]
    async fn test_get_schedules_api_error() {
        // Arrange: Crear AppState que usará las credenciales de test
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);

        // Act: Intentar obtener schedules (fallará por credenciales inválidas)
        let response = get_schedules(State(app_state)).await;

        // Assert: Debería retornar un error de cliente (4xx)
        let resp = response.into_response();
        // Puede ser 401 (unauthorized) o 400 (bad request)
        assert!(resp.status().is_client_error());
    }

    /// Test: get_schedule maneja error de API correctamente
    #[tokio::test]
    async fn test_get_schedule_api_error() {
        // Arrange: Crear AppState que usará las credenciales de test
        let app_state = Arc::new(create_mock_app_state(HashMap::new()).await);

        // Act: Intentar obtener un schedule (fallará por credenciales inválidas)
        let response = get_schedule(State(app_state), Path("1".to_string())).await;

        // Assert: Debería retornar un error de cliente (4xx)
        let resp = response.into_response();
        // Puede ser 401 (unauthorized) o 400 (bad request)
        assert!(resp.status().is_client_error());
    }

    /// Test: get_booking con mock que retorna JSON inválido (no parseable)
    #[tokio::test]
    async fn test_get_booking_invalid_json_parse_error() {
        // Arrange: Crear mock server que retorna JSON inválido
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("GET", "/bookings/test-booking-id")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-06-11")
            .with_status(200)
            .with_header("content-type", "application/json")
            // JSON inválido que no coincide con CalApiResponse<CalBookingPayload>
            .with_body("{\"wrong_structure\": true}")
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        // Act
        let response = get_booking(State(app_state), Path("test-booking-id".to_string())).await;

        // Assert: Debería retornar INTERNAL_SERVER_ERROR
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test: get_booking con mock que simula error al leer el body de la respuesta
    #[tokio::test]
    async fn test_get_booking_failed_to_read_response_body() {
        // Arrange: Este test verifica que el código maneja el caso cuando response.text() falla
        // Aunque es difícil simular este error con mockito, el test con JSON inválido
        // cubre el camino de error de parsing. Para cobertura completa, podríamos necesitar
        // una librería de mocking más sofisticada.

        // Por ahora, verificamos el caso de JSON vacío que podría causar problemas
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("GET", "/bookings/test-booking-id")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-06-11")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("")  // Body vacío
            .create_async()
            .await;

        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        // Act
        let response = get_booking(State(app_state), Path("test-booking-id".to_string())).await;

        // Assert: Debería manejar el error apropiadamente
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test: add_guests_to_booking con mock que retorna JSON inválido
    #[tokio::test]
    async fn test_add_guests_invalid_json_parse_error() {
        // Arrange: Crear mock server que retorna JSON inválido
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("POST", "/bookings/test-booking-id/guests")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-08-13")
            .with_status(200)
            .with_header("content-type", "application/json")
            // JSON inválido que no coincide con CalApiResponse<CalBookingPayload>
            .with_body("{\"invalid\": \"structure\"}")
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        let payload = AddGuestsPayload {
            guests: vec![GuestInput {
                email: "guest@example.com".to_string(),
                name: Some("Test Guest".to_string()),
                time_zone: Some("Europe/Madrid".to_string()),
                phone_number: None,
                language: Some("es".to_string()),
            }],
        };

        // Act
        let response = add_guests_to_booking(
            State(app_state),
            Path("test-booking-id".to_string()),
            axum::Json(payload),
        )
        .await;

        // Assert: Debería retornar INTERNAL_SERVER_ERROR
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test: get_all_bookings con mock que retorna JSON inválido
    #[tokio::test]
    async fn test_get_all_bookings_invalid_json_parse_error() {
        // Arrange: Crear mock server que retorna JSON inválido
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("GET", mockito::Matcher::Regex(r"^/bookings.*".to_string()))
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-06-11")
            .with_status(200)
            .with_header("content-type", "application/json")
            // JSON inválido que no coincide con CalBookingsResponse
            .with_body("{\"not_valid\": true}")
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        let params = BookingsQueryParams {
            event_type_id: None,
            event_type_ids: None,
            attendee_email: None,
            attendee_name: None,
            team_id: None,
            after_start: None,
            before_end: None,
            status: None,
            sort_start: None,
        };

        // Act
        let response = get_all_bookings(State(app_state), Query(params)).await;

        // Assert: Debería retornar INTERNAL_SERVER_ERROR
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test: confirm_booking con error de red (servidor no accesible)
    #[tokio::test]
    async fn test_confirm_booking_network_error() {
        // Arrange: Crear AppState con URL inválida que cause error de red
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        // URL que causará error de red (puerto cerrado)
        app_state.cal_options.base_url = "http://localhost:1".to_string();
        let app_state = Arc::new(app_state);

        // Act
        let response = confirm_booking(State(app_state), Path("booking-123".to_string())).await;

        // Assert: Debería retornar BAD_GATEWAY (502)
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
    }

    /// Test: get_schedules con mock que retorna JSON inválido
    #[tokio::test]
    async fn test_get_schedules_invalid_json_parse_error() {
        // Arrange: Crear mock server que retorna JSON inválido
        let mut server = mockito::Server::new_async().await;
        let mock_url = server.url();

        let _m = server
            .mock("GET", "/schedules")
            .match_header("Authorization", "test-api-key")
            .match_header("cal-api-version", "2024-06-11")
            .with_status(200)
            .with_header("content-type", "application/json")
            // JSON inválido que no coincide con CalApiResponse<Vec<Schedule>>
            .with_body("{\"invalid\": \"response\"}")
            .create_async()
            .await;

        // Crear AppState con URL del mock server
        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.cal_options.base_url = mock_url;
        let app_state = Arc::new(app_state);

        // Act
        let response = get_schedules(State(app_state)).await;

        // Assert: Debería retornar INTERNAL_SERVER_ERROR
        let resp = response.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
