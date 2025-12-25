/// Fixtures: datos de prueba reutilizables
#[cfg(test)]
use crate::models::user::{Provider, UserRequest};

#[cfg(test)]
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

#[cfg(test)]
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
