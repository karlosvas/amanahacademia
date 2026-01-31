/// Fixtures: datos de prueba reutilizables
#[cfg(test)]
pub mod fixtures {
    use crate::models::{
        cal::{BookingStatus, CalBookingPayload},
        metrics::ServiceAccount,
        state::{AppState, CalOptions, CustomFirebase, GAOptions, KeyCache, MailchimpOptions},
        user::{Provider, UserRequest},
    };
    use resend_rs::Resend;
    use std::{collections::HashMap, sync::Arc, time::SystemTime};
    use tokio::sync::RwLock;

    /// Crea un UserRequest válido para tests
    pub fn create_test_user() -> UserRequest {
        UserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            provider: Provider::Email,
            first_free_class: false,
            name: Some("Test User".to_string()),
            phone_number: None,
            id_token: Some("fake-token".to_string()),
            role: None,
            permissions: None,
            subscription_tier: None,
        }
    }

    /// Crea un UserRequest con Google provider
    pub fn create_google_user() -> UserRequest {
        UserRequest {
            email: "google@example.com".to_string(),
            password: "".to_string(), // Google users don't use password but field is required
            provider: Provider::Google,
            first_free_class: false,
            name: Some("Google User".to_string()),
            phone_number: None,
            id_token: Some("google-token".to_string()),
            role: None,
            permissions: None,
            subscription_tier: None,
        }
    }

    /// Crea un AppState mock para tests de Cal.com
    pub async fn create_mock_app_state(
        initial_cache: HashMap<String, CalBookingPayload>,
    ) -> AppState {
        AppState {
            cal_options: CalOptions {
                client: reqwest::Client::new(),
                api_key: "test-api-key".to_string(),
                base_url: "https://api.cal.com/v2".to_string(),
                booking_cache: Arc::new(RwLock::new(initial_cache)),
                recent_changes: Arc::new(RwLock::new(vec![])),
                team_id: "1234".to_string(),
                enable_teams: false,
            },
            firebase_options: CustomFirebase {
                firebase_keys: Arc::new(RwLock::new(KeyCache {
                    keys: serde_json::json!({}),
                    fetched_at: SystemTime::now(),
                })),
                firebase_project_id: "test-project".to_string(),
                firebase_api_key: "test-api-key".to_string(),
                firebase_database_url: "https://test.firebaseio.com".to_string(),
                firebase_database_secret: "test-secret".to_string(),
                firebase_client: reqwest::Client::new(),
            },
            stripe_client: stripe::Client::new("sk_test_key"),
            resend_client: Resend::new("re_test_key"),
            mailchimp_client: MailchimpOptions::new(
                "test-key-us1".to_string(),
                "us1".to_string(),
                "test-list-id".to_string(),
            ),
            ga_options: GAOptions {
                client: reqwest::Client::new(),
                service_account: ServiceAccount {
                    client_email: "test@test.com".to_string(),
                    private_key: "test-key".to_string(),
                },
                base_url: "https://analyticsdata.googleapis.com/v1beta".to_string(),
                property_id: "test-property".to_string(),
            },
        }
    }

    /// Crea un CalBookingPayload de prueba
    pub fn create_test_booking(uid: &str, status: BookingStatus) -> CalBookingPayload {
        CalBookingPayload {
            uid: Some(uid.to_string()),
            status,
            id: Some(1),
            booking_id: Some(1),
            attendees: vec![],
            event_type_id: None,
            event_type_slug: None,
            event_type: None,
            user: None,
            team_slug: None,
            organization_slug: None,
            title: None,
            description: None,
            start_time: None,
            end_time: None,
            duration: None,
            organizer: None,
            location: None,
            metadata: None,
            cancellation_reason: None,
            meeting_url: None,
            cancel_token: None,
            reschedule_token: None,
            hosts: None,
            seat_uid: None,
            absent_host: None,
            created_at: None,
            updated_at: None,
            rating: None,
            ics_uid: None,
            rescheduled_by_email: None,
            guests: None,
        }
    }
}
