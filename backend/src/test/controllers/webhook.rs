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
}
