#[cfg(test)]
mod tests {
    use {
        crate::{
            controllers::mailchimp::{add_contact, get_all_contacts},
            models::{
                mailchimp::{Contact, MergeFields},
                state::AppState,
            },
            test_fixtures::fixtures::create_mock_app_state,
        },
        axum::{
            extract::{Json, State},
            http::StatusCode,
            response::IntoResponse,
        },
        serde_json::{Value, json},
        std::{collections::HashMap, sync::Arc},
    };

    async fn state_with_mailchimp(url: String) -> Arc<AppState> {
        let mut state = create_mock_app_state(HashMap::new()).await;
        state.mailchimp_client.base_url_override = Some(url);
        Arc::new(state)
    }

    fn contact(email: &str) -> Contact {
        Contact {
            email_address: email.to_string(),
            status: "subscribed".to_string(),
            merge_fields: Some(MergeFields {
                fname: Some("Ana".to_string()),
                lname: None,
            }),
        }
    }

    async fn add(state: Arc<AppState>, email: &str) -> (StatusCode, Value) {
        let response = add_contact(State(state), Json(contact(email)))
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

    fn member_body(status: &str) -> String {
        json!({
            "id": "abc",
            "email_address": "ana@example.com",
            "status": status
        })
        .to_string()
    }

    #[tokio::test]
    async fn test_add_contact_subscribed() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PUT", "/lists/test-list-id/members/ana@example.com")
            .with_status(200)
            .with_body(member_body("subscribed"))
            .create_async()
            .await;
        let state = state_with_mailchimp(server.url()).await;

        let (status, body) = add(state, "  ANA@example.com ").await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["message"], "Contacto añadido correctamente");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_add_contact_pending() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("PUT", "/lists/test-list-id/members/ana@example.com")
            .with_status(200)
            .with_body(member_body("pending"))
            .create_async()
            .await;
        let state = state_with_mailchimp(server.url()).await;

        let (status, body) = add(state, "ana@example.com").await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["message"], "Contacto añadido correctamente");
    }

    #[tokio::test]
    async fn test_add_contact_existing_unsubscribed() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("PUT", "/lists/test-list-id/members/ana@example.com")
            .with_status(200)
            .with_body(member_body("unsubscribed"))
            .create_async()
            .await;
        let state = state_with_mailchimp(server.url()).await;

        let (status, body) = add(state, "ana@example.com").await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["message"], "El contacto ya existía y no está suscrito");
    }

    #[tokio::test]
    async fn test_add_contact_propagates_mailchimp_error() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("PUT", "/lists/test-list-id/members/ana@example.com")
            .with_status(400)
            .with_body(json!({ "title": "Invalid Resource" }).to_string())
            .create_async()
            .await;
        let state = state_with_mailchimp(server.url()).await;

        let (status, body) = add(state, "ana@example.com").await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body["error"], "Invalid Resource");
    }

    #[tokio::test]
    async fn test_add_contact_unreachable() {
        let state = state_with_mailchimp("http://127.0.0.1:1".to_string()).await;

        let (status, _) = add(state, "ana@example.com").await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_all_contacts_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/lists/test-list-id/members")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(
                json!({ "members": [], "list_id": "test-list-id", "total_items": 0 }).to_string(),
            )
            .create_async()
            .await;
        let state = state_with_mailchimp(server.url()).await;

        let status = get_all_contacts(State(state))
            .await
            .into_response()
            .status();

        assert_eq!(status, StatusCode::OK);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_all_contacts_error_status() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/lists/test-list-id/members")
            .match_query(mockito::Matcher::Any)
            .with_status(401)
            .with_body("unauthorized")
            .create_async()
            .await;
        let state = state_with_mailchimp(server.url()).await;

        let status = get_all_contacts(State(state))
            .await
            .into_response()
            .status();

        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_all_contacts_unreachable() {
        let state = state_with_mailchimp("http://127.0.0.1:1".to_string()).await;

        let status = get_all_contacts(State(state))
            .await
            .into_response()
            .status();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
