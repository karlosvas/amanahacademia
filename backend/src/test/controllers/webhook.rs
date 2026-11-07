#[cfg(test)]
mod tests {
    use {
        crate::{
            controllers::webhook::{handle_cal_webhook, handle_stripe_webhook},
            test_fixtures::fixtures::create_mock_app_state,
        },
        axum::{
            body::Bytes,
            extract::State,
            http::{HeaderMap, HeaderValue, StatusCode},
            response::IntoResponse,
        },
        hmac::{Hmac, Mac},
        sha2::Sha256,
        std::{collections::HashMap, sync::Arc},
    };

    fn sign(secret: &str, message: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    fn header(name: &'static str, value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(name, HeaderValue::from_str(value).unwrap());
        headers
    }

    #[tokio::test]
    async fn test_cal_webhook_rejects_missing_signature() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let body = Bytes::from_static(br#"{"triggerEvent":"BOOKING_CANCELLED"}"#);

        let status = handle_cal_webhook(State(state), HeaderMap::new(), body)
            .await
            .into_response()
            .status();

        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_cal_webhook_rejects_invalid_signature() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let body = r#"{"triggerEvent":"BOOKING_CANCELLED"}"#;

        let status = handle_cal_webhook(
            State(state),
            header("x-cal-signature-256", &sign("other_secret", body)),
            Bytes::from(body),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_cal_webhook_unavailable_without_secret() {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.payment_options.cal_webhook_secret = None;

        let status = handle_cal_webhook(
            State(Arc::new(state)),
            HeaderMap::new(),
            Bytes::from_static(b"{}"),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_cal_webhook_accepts_valid_signature() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let body = r#"{"triggerEvent":"BOOKING_CANCELLED","payload":{}}"#;

        let status = handle_cal_webhook(
            State(state),
            header("x-cal-signature-256", &sign("cal_test", body)),
            Bytes::from(body),
        )
        .await
        .into_response()
        .status();

        assert_ne!(status, StatusCode::UNAUTHORIZED);
        assert_ne!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_stripe_webhook_rejects_invalid_signature() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let body = r#"{"type":"payment_intent.succeeded"}"#;
        let now = chrono::Utc::now().timestamp();
        let signature = sign("whsec_other", &format!("{}.{}", now, body));

        let status = handle_stripe_webhook(
            State(state),
            header("stripe-signature", &format!("t={},v1={}", now, signature)),
            Bytes::from(body),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_stripe_webhook_unavailable_without_secret() {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.payment_options.stripe_webhook_secret = None;

        let status = handle_stripe_webhook(
            State(Arc::new(state)),
            HeaderMap::new(),
            Bytes::from_static(b"{}"),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_stripe_webhook_ignores_other_signed_events() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let body = r#"{"type":"charge.refunded","data":{"object":{"id":"ch_1"}}}"#;
        let now = chrono::Utc::now().timestamp();
        let signature = sign("whsec_test", &format!("{}.{}", now, body));

        let status = handle_stripe_webhook(
            State(state),
            header("stripe-signature", &format!("t={},v1={}", now, signature)),
            Bytes::from(body),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::OK);
    }

    async fn signed_stripe_status(body: &str) -> StatusCode {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let now = chrono::Utc::now().timestamp();
        let signature = sign("whsec_test", &format!("{}.{}", now, body));

        handle_stripe_webhook(
            State(state),
            header("stripe-signature", &format!("t={},v1={}", now, signature)),
            Bytes::from(body.to_string()),
        )
        .await
        .into_response()
        .status()
    }

    #[tokio::test]
    async fn test_stripe_webhook_rejects_invalid_json() {
        assert_eq!(
            signed_stripe_status("not json").await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_stripe_webhook_rejects_missing_payment_intent_id() {
        let body = r#"{"type":"payment_intent.succeeded","data":{"object":{}}}"#;
        assert_eq!(signed_stripe_status(body).await, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cal_webhook_rejects_invalid_payload() {
        let state = Arc::new(create_mock_app_state(HashMap::new()).await);
        let body = "not json";

        let status = handle_cal_webhook(
            State(state),
            header("x-cal-signature-256", &sign("cal_test", body)),
            Bytes::from(body),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    async fn state_with_mocks(url: String) -> Arc<crate::models::state::AppState> {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.cal_options.base_url = url.clone();
        state.firebase_options.firebase_database_url = url;
        Arc::new(state)
    }

    async fn post_cal_event(
        state: Arc<crate::models::state::AppState>,
        trigger: &str,
        payload: serde_json::Value,
    ) -> StatusCode {
        let body = serde_json::json!({
            "triggerEvent": trigger,
            "createdAt": "2026-01-01T00:00:00Z",
            "payload": payload,
        })
        .to_string();

        handle_cal_webhook(
            State(state),
            header("x-cal-signature-256", &sign("cal_test", &body)),
            Bytes::from(body),
        )
        .await
        .into_response()
        .status()
    }

    async fn default_state() -> Arc<crate::models::state::AppState> {
        Arc::new(create_mock_app_state(HashMap::new()).await)
    }

    #[tokio::test]
    async fn test_cal_cancelled_without_uid_is_bad_request() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_CANCELLED",
            serde_json::json!({ "status": "cancelled", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cal_cancelled_with_unsafe_uid_is_rejected() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_CANCELLED",
            serde_json::json!({ "uid": "../etc", "status": "cancelled", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_cal_cancelled_without_payments_succeeds() {
        let mut server = mockito::Server::new_async().await;
        let _relation = server
            .mock("GET", "/relation_cal_stripe/b1.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body("null")
            .create_async()
            .await;

        let status = post_cal_event(
            state_with_mocks(server.url()).await,
            "BOOKING_CANCELLED",
            serde_json::json!({ "uid": "b1", "status": "cancelled", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::OK);
    }

    async fn cancelled_with_payment(booking_status: &str) -> StatusCode {
        let mut server = mockito::Server::new_async().await;
        let _relation = server
            .mock("GET", "/relation_cal_stripe/b1.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(
                serde_json::json!({ "pi_1": { "email": "a@b.com", "amount": 1000 } }).to_string(),
            )
            .create_async()
            .await;
        let _booking = server
            .mock("GET", "/bookings/b1")
            .with_status(200)
            .with_body(
                serde_json::json!({
                    "status": "success",
                    "data": {
                        "uid": "b1",
                        "status": booking_status,
                        "eventType": { "id": 1, "slug": "standard-class" },
                        "attendees": []
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        post_cal_event(
            state_with_mocks(server.url()).await,
            "BOOKING_CANCELLED",
            serde_json::json!({ "uid": "b1", "status": "cancelled", "attendees": [] }),
        )
        .await
    }

    #[tokio::test]
    async fn test_cal_cancelled_booking_not_cancelled_in_cal_is_not_refunded() {
        assert_eq!(cancelled_with_payment("accepted").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_cal_cancelled_with_auto_refunds_disabled_succeeds() {
        assert_eq!(cancelled_with_payment("cancelled").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_cal_cancelled_firebase_failure_is_internal_error() {
        let mut server = mockito::Server::new_async().await;
        let _relation = server
            .mock("GET", "/relation_cal_stripe/b1.json")
            .match_query(mockito::Matcher::Any)
            .with_status(500)
            .create_async()
            .await;

        let status = post_cal_event(
            state_with_mocks(server.url()).await,
            "BOOKING_CANCELLED",
            serde_json::json!({ "uid": "b1", "status": "cancelled", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_cal_created_without_uid_is_bad_request() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_CREATED",
            serde_json::json!({ "status": "accepted", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cal_created_free_class_without_attendees_fails() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_CREATED",
            serde_json::json!({
                "uid": "b1",
                "status": "accepted",
                "type": "free-class",
                "attendees": []
            }),
        )
        .await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_cal_created_paid_class_is_not_processed() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_CREATED",
            serde_json::json!({
                "uid": "b1",
                "status": "accepted",
                "type": "standard-class",
                "attendees": []
            }),
        )
        .await;

        assert_eq!(status, StatusCode::NOT_ACCEPTABLE);
    }

    #[tokio::test]
    async fn test_cal_paid_event_is_not_processed() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_PAID",
            serde_json::json!({ "uid": "b1", "status": "accepted", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::NOT_ACCEPTABLE);
    }

    #[tokio::test]
    async fn test_cal_rescheduled_without_uid_is_bad_request() {
        let status = post_cal_event(
            default_state().await,
            "BOOKING_RESCHEDULED",
            serde_json::json!({ "status": "accepted", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cal_rescheduled_without_original_booking_succeeds() {
        let mut server = mockito::Server::new_async().await;
        let _booking = server
            .mock("GET", "/bookings/b2")
            .with_status(200)
            .with_body(
                serde_json::json!({
                    "status": "success",
                    "data": {
                        "uid": "b2",
                        "status": "accepted",
                        "eventType": { "id": 1, "slug": "standard-class" },
                        "attendees": []
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let status = post_cal_event(
            state_with_mocks(server.url()).await,
            "BOOKING_RESCHEDULED",
            serde_json::json!({ "uid": "b2", "status": "accepted", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_cal_rescheduled_booking_fetch_failure_is_internal_error() {
        let mut server = mockito::Server::new_async().await;
        let _booking = server
            .mock("GET", "/bookings/b2")
            .with_status(404)
            .with_body("not found")
            .create_async()
            .await;

        let status = post_cal_event(
            state_with_mocks(server.url()).await,
            "BOOKING_RESCHEDULED",
            serde_json::json!({ "uid": "b2", "status": "accepted", "attendees": [] }),
        )
        .await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        assert_eq!(crate::controllers::webhook::health_check().await, "OK");
    }
}
