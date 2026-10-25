#[cfg(test)]
mod tests {
    use {
        crate::{
            controllers::payments::payment_intent,
            models::{
                firebase::UserAuthentication,
                stripe::{CheckoutPayload, PriceTable},
            },
            test_fixtures::fixtures::create_mock_app_state,
        },
        axum::{
            Extension, Json,
            extract::State,
            http::{HeaderMap, HeaderValue, StatusCode},
            response::IntoResponse,
        },
        serde_json::{Value, json},
        std::{collections::HashMap, sync::Arc},
    };

    const PRICES: PriceTable = PriceTable {
        individual_standard: 30.0,
        individual_conversation: 25.0,
        group: 10.0,
    };

    fn claims() -> UserAuthentication {
        UserAuthentication {
            sub: "user-1".to_string(),
            iss: "https://securetoken.google.com/amanahacademia".to_string(),
            aud: "amanahacademia".to_string(),
            iat: 0,
            exp: 0,
            email: Some("student@example.com".to_string()),
            email_verified: Some(true),
            name: Some("Student".to_string()),
            picture: None,
            auth_time: 0,
            user_id: "user-1".to_string(),
            firebase: None,
            phone_number: None,
            provider_id: None,
        }
    }

    fn internal_headers(secret: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("x-internal-secret", HeaderValue::from_str(secret).unwrap());
        headers
    }

    fn booking_payload(uid: &str) -> CheckoutPayload {
        CheckoutPayload {
            booking_uid: Some(uid.to_string()),
            class_type: None,
            weeks: None,
            prices: PRICES,
        }
    }

    async fn call(
        state: Arc<crate::models::state::AppState>,
        headers: HeaderMap,
        payload: CheckoutPayload,
    ) -> StatusCode {
        payment_intent(State(state), Extension(claims()), headers, Json(payload))
            .await
            .into_response()
            .status()
    }

    async fn state_with_cal(base_url: String) -> Arc<crate::models::state::AppState> {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.cal_options.base_url = base_url;
        Arc::new(state)
    }

    #[tokio::test]
    async fn test_rejects_requests_without_internal_secret() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);

        assert_eq!(
            call(state.clone(), HeaderMap::new(), booking_payload("uid")).await,
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            call(state, internal_headers("wrong"), booking_payload("uid")).await,
            StatusCode::FORBIDDEN
        );
    }

    #[tokio::test]
    async fn test_unavailable_when_internal_secret_not_configured() {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.payment_options.internal_api_secret = None;

        assert_eq!(
            call(
                Arc::new(state),
                internal_headers("internal_test"),
                booking_payload("uid")
            )
            .await,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[tokio::test]
    async fn test_rejects_payload_without_booking_or_weeks() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = CheckoutPayload {
            booking_uid: None,
            class_type: Some("group-class".to_string()),
            weeks: None,
            prices: PRICES,
        };

        assert_eq!(
            call(state, internal_headers("internal_test"), payload).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_rejects_unsafe_booking_id() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("../user_profiles")
            )
            .await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_rejects_invalid_weeks_flow() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let payload = CheckoutPayload {
            booking_uid: None,
            class_type: Some("free-class".to_string()),
            weeks: Some(4),
            prices: PRICES,
        };

        assert_eq!(
            call(state.clone(), internal_headers("internal_test"), payload).await,
            StatusCode::BAD_REQUEST
        );

        let payload = CheckoutPayload {
            booking_uid: None,
            class_type: Some("group-class".to_string()),
            weeks: Some(40),
            prices: PRICES,
        };
        assert_eq!(
            call(state, internal_headers("internal_test"), payload).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_rejects_cancelled_booking() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/bookings/booking-1")
            .with_status(200)
            .with_body(
                json!({
                    "status": "success",
                    "data": {
                        "uid": "booking-1",
                        "status": "cancelled",
                        "eventType": { "id": 1, "slug": "standard-class" },
                        "attendees": []
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let state = state_with_cal(server.url()).await;

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("booking-1")
            )
            .await,
            StatusCode::CONFLICT
        );
    }

    #[tokio::test]
    async fn test_rejects_free_class_booking() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/bookings/booking-1")
            .with_status(200)
            .with_body(
                json!({
                    "status": "success",
                    "data": {
                        "uid": "booking-1",
                        "status": "accepted",
                        "eventType": { "id": 1, "slug": "free-class" },
                        "attendees": []
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let state = state_with_cal(server.url()).await;

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("booking-1")
            )
            .await,
            StatusCode::BAD_REQUEST
        );
    }

    async fn mock_booking(server: &mut mockito::ServerGuard, slug: &str) -> mockito::Mock {
        server
            .mock("GET", "/bookings/booking-1")
            .with_status(200)
            .with_body(
                json!({
                    "status": "success",
                    "data": {
                        "uid": "booking-1",
                        "status": "accepted",
                        "eventType": { "id": 1, "slug": slug },
                        "attendees": [{
                            "name": "Student",
                            "email": "student@example.com",
                            "timeZone": "Europe/Madrid"
                        }]
                    }
                })
                .to_string(),
            )
            .create_async()
            .await
    }

    async fn mock_relation(server: &mut mockito::ServerGuard, relation: Value) -> mockito::Mock {
        server
            .mock("GET", "/relation_cal_stripe/booking-1.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(relation.to_string())
            .create_async()
            .await
    }

    async fn state_with_mocks(url: String) -> Arc<crate::models::state::AppState> {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.cal_options.base_url = url.clone();
        state.firebase_options.firebase_database_url = url;
        Arc::new(state)
    }

    #[tokio::test]
    async fn test_rejects_group_class_already_paid_by_same_student() {
        let mut server = mockito::Server::new_async().await;
        let _booking = mock_booking(&mut server, "group-class").await;
        let _relation = mock_relation(
            &mut server,
            json!({ "pi_1": { "email": "STUDENT@example.com", "amount": 1000 } }),
        )
        .await;

        let state = state_with_mocks(server.url()).await;

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("booking-1")
            )
            .await,
            StatusCode::CONFLICT
        );
    }

    #[tokio::test]
    async fn test_rejects_individual_booking_already_paid() {
        let mut server = mockito::Server::new_async().await;
        let _booking = mock_booking(&mut server, "standard-class").await;
        let _relation = mock_relation(&mut server, json!({ "stripe_id": "pi_legacy" })).await;

        let state = state_with_mocks(server.url()).await;

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("booking-1")
            )
            .await,
            StatusCode::CONFLICT
        );
    }

    #[tokio::test]
    async fn test_rejects_when_payments_cannot_be_verified() {
        let mut server = mockito::Server::new_async().await;
        let _booking = mock_booking(&mut server, "standard-class").await;
        let _relation = server
            .mock("GET", "/relation_cal_stripe/booking-1.json")
            .match_query(mockito::Matcher::Any)
            .with_status(500)
            .create_async()
            .await;

        let state = state_with_mocks(server.url()).await;

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("booking-1")
            )
            .await,
            StatusCode::BAD_GATEWAY
        );
    }

    #[tokio::test]
    async fn test_rejects_booking_when_cal_fails() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/bookings/booking-1")
            .with_status(500)
            .create_async()
            .await;

        let state = state_with_cal(server.url()).await;

        assert_eq!(
            call(
                state,
                internal_headers("internal_test"),
                booking_payload("booking-1")
            )
            .await,
            StatusCode::BAD_GATEWAY
        );
    }
}
