#[cfg(test)]
mod tests {
    use {
        crate::utils::validations::{ValidatedJson, validate_non_whitespace},
        axum::{
            Json, Router,
            body::Body,
            extract::Request,
            http::{Method, StatusCode},
            routing::post,
        },
        serde::{Deserialize, Serialize},
        tower::ServiceExt,
        validator::Validate,
    };

    #[test]
    fn test_validate_non_whitespace_valid_string() {
        let result = validate_non_whitespace("hello");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_whitespace_valid_with_leading_spaces() {
        let result = validate_non_whitespace("  hello");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_whitespace_valid_with_trailing_spaces() {
        let result = validate_non_whitespace("hello  ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_whitespace_valid_with_both_spaces() {
        let result = validate_non_whitespace("  hello world  ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_whitespace_empty_string() {
        let result = validate_non_whitespace("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "cannot_be_empty");
    }

    #[test]
    fn test_validate_non_whitespace_only_spaces() {
        let result = validate_non_whitespace("     ");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "cannot_be_empty");
    }

    #[test]
    fn test_validate_non_whitespace_only_tabs() {
        let result = validate_non_whitespace("\t\t\t");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "cannot_be_empty");
    }

    #[test]
    fn test_validate_non_whitespace_mixed_whitespace() {
        let result = validate_non_whitespace(" \t \n \r ");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "cannot_be_empty");
    }

    #[test]
    fn test_validate_non_whitespace_single_character() {
        let result = validate_non_whitespace("a");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_whitespace_unicode() {
        let result = validate_non_whitespace("مرحبا");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_whitespace_special_characters() {
        let result = validate_non_whitespace("!@#$%");
        assert!(result.is_ok());
    }

    mod validated_json_tests {
        use super::*;

        /// Estructura de prueba que usa Validate
        #[derive(Debug, Deserialize, Serialize, Validate)]
        struct TestPayload {
            #[validate(length(min = 1, max = 100))]
            name: String,
            #[validate(range(min = 0, max = 150))]
            age: i32,
            #[validate(email)]
            email: String,
        }

        /// Handler de prueba que usa ValidatedJson
        async fn test_handler(
            ValidatedJson(payload): ValidatedJson<TestPayload>,
        ) -> Json<TestPayload> {
            Json(payload)
        }

        #[tokio::test]
        async fn test_validated_json_with_valid_payload() {
            let app = Router::new().route("/test", post(test_handler));

            let body = r#"{
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com"
    }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
        }

        #[tokio::test]
        async fn test_validated_json_with_invalid_email() {
            let app = Router::new().route("/test", post(test_handler));

            let body = r#"{
        "name": "John Doe",
        "age": 30,
        "email": "invalid-email"
    }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_validated_json_with_invalid_age() {
            let app = Router::new().route("/test", post(test_handler));

            let body = r#"{
        "name": "John Doe",
        "age": 200,
        "email": "john@example.com"
    }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_validated_json_with_empty_name() {
            let app = Router::new().route("/test", post(test_handler));

            let body = r#"{
        "name": "",
        "age": 30,
        "email": "john@example.com"
    }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_validated_json_with_invalid_json() {
            let app = Router::new().route("/test", post(test_handler));

            let body = r#"{ invalid json }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_validated_json_with_missing_fields() {
            let app = Router::new().route("/test", post(test_handler));

            let body = r#"{
        "name": "John Doe"
    }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[tokio::test]
        async fn test_validated_json_with_all_edge_cases() {
            let app = Router::new().route("/test", post(test_handler));

            // Edad mínima válida (0)
            let body = r#"{
        "name": "A",
        "age": 0,
        "email": "a@b.co"
    }"#;

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            // Edad máxima válida (150)
            let body = r#"{
        "name": "B",
        "age": 150,
        "email": "b@c.com"
    }"#;

            let response = app
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/test")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
