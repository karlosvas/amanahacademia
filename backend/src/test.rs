/// Tests básicos y útiles para la aplicación
#[cfg(test)]
mod tests {
    use crate::models::user::{Provider, UserRequest};
    use serde_json;

    #[test]
    fn provider_serde_and_as_str() {
        let p: Provider = serde_json::from_str(r#""email""#).expect("deserialize provider");
        assert_eq!(p, Provider::Email);
        assert_eq!(
            match p {
                Provider::Email => "email",
                Provider::Google => "google",
            },
            "email"
        );
        let s: String = serde_json::to_string(&Provider::Google).expect("serialize provider");
        assert_eq!(s, r#""google""#);
    }

    #[test]
    fn userrequest_deserialize_and_roundtrip() {
        let json = r#"
        {
            "email": "test@example.com",
            "password": "secret",
            "provider": "google",
            "first_free_class": false,
            "name": "Test User",
            "phone_number": null,
            "id_token": "fake-token",
            "role": null,
            "permissions": null,
            "subscription_tier": null
        }"#;

        let req: UserRequest = serde_json::from_str(json).expect("deserialize UserRequest");
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.provider, Provider::Google);
        assert_eq!(req.name.as_deref(), Some("Test User"));

        // Roundtrip: serializar y volver a parsear produce el mismo email
        let round: String = serde_json::to_string(&req).expect("serialize UserRequest");
        let parsed: UserRequest = serde_json::from_str(&round).expect("roundtrip");
        assert_eq!(parsed.email, req.email);
    }
}
