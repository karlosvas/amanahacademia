#[cfg(test)]
mod tests {
    use {
        crate::{
            controllers::metrics::{get_article_metrics, get_class_metrics, get_user_metrics},
            models::{metrics::GAToken, state::AppState},
            test_fixtures::fixtures::create_mock_app_state,
        },
        axum::{Extension, extract::State, response::IntoResponse},
        mockito::{Mock, ServerGuard},
        serde_json::json,
        std::{collections::HashMap, sync::Arc},
    };

    #[tokio::test]
    async fn test_get_user_metrics_success() {
        let mut server: ServerGuard = mockito::Server::new_async().await;

        let _m: Mock = server
            .mock("POST", "/properties/test-property:runReport")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "rows": [],
                    "rowCount": 0,
                    "metadata": {},
                    "kind": "analyticsData#runReport"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut app_state: AppState = create_mock_app_state(HashMap::new()).await;
        app_state.ga_options.property_id = "test-property".to_string();
        app_state.ga_options.base_url = server.url();
        let state = Arc::new(app_state);

        let response = get_user_metrics(State(state), Extension(GAToken("test_token".to_string())))
            .await
            .into_response();

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_get_user_metrics_api_error() {
        let mut server = mockito::Server::new_async().await;

        let _m: Mock = server
            .mock("POST", "/properties/test-property:runReport")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(json!({"error": "Unauthorized"}).to_string())
            .create_async()
            .await;

        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.ga_options.property_id = "test-property".to_string();
        app_state.ga_options.base_url = server.url();
        let state = Arc::new(app_state);

        let response = get_user_metrics(
            State(state),
            Extension(GAToken("invalid_token".to_string())),
        )
        .await
        .into_response();

        assert_eq!(response.status(), 500);
    }

    #[tokio::test]
    async fn test_get_article_metrics_success() {
        let mut server = mockito::Server::new_async().await;

        let _m = server
            .mock("POST", "/properties/test-property:runReport")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "rows": [{"dimensionValues": [{"value": "article_click"}], "metricValues": [{"value": "42"}]}],
                    "rowCount": 1,
                    "metadata": {},
                    "kind": "analyticsData#runReport"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut app_state = create_mock_app_state(HashMap::new()).await;
        app_state.ga_options.property_id = "test-property".to_string();
        app_state.ga_options.base_url = server.url();
        let state = Arc::new(app_state);

        let response =
            get_article_metrics(State(state), Extension(GAToken("test_token".to_string())))
                .await
                .into_response();

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_get_article_metrics_network_error() {
        let mut app_state: AppState = create_mock_app_state(HashMap::new()).await;
        app_state.ga_options.property_id = "test-property".to_string();
        app_state.ga_options.base_url = "http://localhost:1".to_string();
        let state: Arc<AppState> = Arc::new(app_state);

        let response =
            get_article_metrics(State(state), Extension(GAToken("test_token".to_string())))
                .await
                .into_response();

        assert_eq!(response.status(), 500);
    }

    #[tokio::test]
    async fn test_get_class_metrics_success() {
        let mut server: ServerGuard = mockito::Server::new_async().await;

        let _m = server
            .mock("POST", "/properties/test-property:runReport")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "rows": [{"dimensionValues": [{"value": "class_booking"}], "metricValues": [{"value": "15"}]}],
                    "rowCount": 1,
                    "metadata": {},
                    "kind": "analyticsData#runReport"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut app_state: AppState = create_mock_app_state(HashMap::new()).await;
        app_state.ga_options.property_id = "test-property".to_string();
        app_state.ga_options.base_url = server.url();
        let state = Arc::new(app_state);

        let response =
            get_class_metrics(State(state), Extension(GAToken("test_token".to_string())))
                .await
                .into_response();

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_get_class_metrics_invalid_json_response() {
        let mut server: ServerGuard = mockito::Server::new_async().await;

        let _m = server
            .mock("POST", "/properties/test-property:runReport")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{invalid json}")
            .create_async()
            .await;

        let mut app_state: AppState = create_mock_app_state(HashMap::new()).await;
        app_state.ga_options.property_id = "test-property".to_string();
        app_state.ga_options.base_url = server.url();
        let state = Arc::new(app_state);

        let response =
            get_class_metrics(State(state), Extension(GAToken("test_token".to_string())))
                .await
                .into_response();

        assert_eq!(response.status(), 500);
    }
}
