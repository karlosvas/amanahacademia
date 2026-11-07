#[cfg(test)]
mod tests {
    use {
        crate::{
            controllers::sourvey::{create_survey, get_all_survey_results, get_survey_results},
            models::{
                sourvey::{Question, QuestionType, Survey},
                state::AppState,
            },
            test_fixtures::fixtures::create_mock_app_state,
        },
        axum::{
            Extension, Json,
            extract::{Path, State},
            http::StatusCode,
            response::IntoResponse,
        },
        serde_json::{Value, json},
        std::{collections::HashMap, sync::Arc},
    };

    async fn state_with_firebase(url: String) -> Arc<AppState> {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.firebase_options.firebase_database_url = url;
        Arc::new(state)
    }

    fn survey(id: &str) -> Survey {
        Survey {
            id: id.to_string(),
            title: "Encuesta".to_string(),
            description: "Descripcion".to_string(),
            user_email: "user@example.com".to_string(),
            submitted_at: None,
            questions: vec![Question {
                id: "q1".to_string(),
                label: "Nivel".to_string(),
                question_type: QuestionType::Text,
                options: None,
                required: true,
                answer: Some("A1".to_string()),
            }],
        }
    }

    fn surveys_body() -> String {
        json!({ "s1": survey("s1"), "s2": survey("s2") }).to_string()
    }

    async fn get_one(state: Arc<AppState>, id: &str) -> (StatusCode, Value) {
        let response = get_survey_results(
            Path(id.to_string()),
            State(state),
            Extension("token".to_string()),
        )
        .await
        .into_response();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    async fn get_all(state: Arc<AppState>) -> (StatusCode, Value) {
        let response = get_all_survey_results(State(state), Extension("token".to_string()))
            .await
            .into_response();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }

    #[tokio::test]
    async fn test_create_survey_generates_id_and_timestamp() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PUT", mockito::Matcher::Regex(r"^/surveys/.+\.json".into()))
            .with_status(200)
            .with_body(
                json!({
                    "id": "generated",
                    "title": "Encuesta",
                    "description": "Descripcion",
                    "userEmail": "user@example.com",
                    "submittedAt": "2026-01-01T00:00:00Z",
                    "questions": []
                })
                .to_string(),
            )
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let response = create_survey(
            State(state),
            Extension("token".to_string()),
            Json(survey("  ")),
        )
        .await
        .into_response();

        assert_eq!(response.status(), StatusCode::CREATED);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_survey_keeps_existing_id() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PUT", "/surveys/fixed.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(json!(survey("fixed")).to_string())
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let status = create_survey(
            State(state),
            Extension("token".to_string()),
            Json(survey("fixed")),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::CREATED);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_create_survey_propagates_firebase_error() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("PUT", "/surveys/fixed.json")
            .match_query(mockito::Matcher::Any)
            .with_status(401)
            .with_body(json!({ "error": "Permission denied" }).to_string())
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let status = create_survey(
            State(state),
            Extension("token".to_string()),
            Json(survey("fixed")),
        )
        .await
        .into_response()
        .status();

        assert_ne!(status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_survey_unparseable_firebase_response() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("PUT", "/surveys/fixed.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body("not json")
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let status = create_survey(
            State(state),
            Extension("token".to_string()),
            Json(survey("fixed")),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_create_survey_firebase_unreachable() {
        let state = state_with_firebase("http://127.0.0.1:1".to_string()).await;

        let status = create_survey(
            State(state),
            Extension("token".to_string()),
            Json(survey("fixed")),
        )
        .await
        .into_response()
        .status();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_survey_results_latest_returns_all() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(surveys_body())
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, body) = get_one(state, "latest").await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_survey_results_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(surveys_body())
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, body) = get_one(state, "s1").await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_survey_results_unknown_id_is_empty() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(surveys_body())
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, body) = get_one(state, "missing").await;

        assert_eq!(status, StatusCode::OK);
        assert!(body["data"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_survey_results_null_body_is_empty() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body("null")
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, body) = get_one(state, "latest").await;

        assert_eq!(status, StatusCode::OK);
        assert!(body["data"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_survey_results_invalid_json() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body("{\"s1\": 5}")
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, _) = get_one(state, "latest").await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_survey_results_firebase_error_status() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(500)
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, _) = get_one(state, "latest").await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_survey_results_unreachable() {
        let state = state_with_firebase("http://127.0.0.1:1".to_string()).await;

        let (status, _) = get_one(state, "latest").await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_all_survey_results_success() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(surveys_body())
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, body) = get_all(state).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_all_survey_results_empty_body() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body("")
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, body) = get_all(state).await;

        assert_eq!(status, StatusCode::OK);
        assert!(body["data"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_all_survey_results_invalid_json() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body("{\"s1\": 5}")
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, _) = get_all(state).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_all_survey_results_firebase_error_status() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/surveys.json")
            .match_query(mockito::Matcher::Any)
            .with_status(503)
            .create_async()
            .await;
        let state = state_with_firebase(server.url()).await;

        let (status, _) = get_all(state).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_all_survey_results_unreachable() {
        let state = state_with_firebase("http://127.0.0.1:1".to_string()).await;

        let (status, _) = get_all(state).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
