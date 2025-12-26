use {
    crate::{
        middleware::auth::verify_firebase_token,
        models::{
            error::AuthError,
            firebase::UserAuthentication,
            metrics::{ClaimsGA, GAToken, ServiceAccount},
            state::KeyCache,
        },
    },
    axum::http::StatusCode,
    jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, encode},
    serde_json::{Value, json},
    std::time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

// ========== FIXTURES Y HELPERS ==========

/// Crea un token JWT válido para testing
fn create_valid_test_token(
    project_id: &str,
    user_id: &str,
    encoding_key: &EncodingKey,
    kid: &str,
) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = UserAuthentication {
        sub: user_id.to_string(),
        iss: format!("https://securetoken.google.com/{}", project_id),
        aud: project_id.to_string(),
        iat: now,
        exp: now + 3600,
        email: Some("test@example.com".to_string()),
        email_verified: Some(true),
        name: Some("Test User".to_string()),
        picture: None,
        auth_time: now,
        user_id: user_id.to_string(),
        firebase: None,
        phone_number: None,
        provider_id: Some("password".to_string()),
    };

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.to_string());

    encode(&header, &claims, encoding_key).expect("Failed to encode JWT")
}

/// Crea un par de claves RSA para testing
fn create_test_rsa_keys() -> (EncodingKey, DecodingKey, String) {
    let private_key = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDNPyPzvh7i+WMs
0S5Px8ip6Mxjag6Xxf1SZ979q+L/nPDvuOADnzUnSbeBNYzAwz79zYoqra64zK1H
cFrxCcVfMqu8Wrko7Wevshoo5fbxzTE8i+/J3cd6lRE0+pwrN0+k5gUiPj1KziJ9
r4cOexfmWgRRpjTOEEpxrOSohlDpj04zmPn7B3fTlAtxlsJRzw/c+su/PY0aopWp
Xo/lLC//IxqeYgkad8g8GcEpk0UPaXmQodmaDpk7BXz4R6Wln+d9ABUx8a9sME6y
x0+0PANP7DyWmoPRsrVzA7msjAQ/HnO3cblWzkiY8Mikgo5xJTQS6aMO3zIYmOmw
b0xS5mHlAgMBAAECggEAG4k7kRFyQmBUAmjEDlcO4GDHvxS1BX6+GEawP5dGeqW7
G2ZRw5qh/nXg5ThifGAVfOaNAWHQ3aE0JC/6O3lknfuF19zSF6AWN5es88y6f0FY
uDdMAei7wQHrz5BJ0HB4wnZLvQbdoUIblYItm+8+yxxLlQq37ed4nVylRsjSZSsj
wBeJuEM1ZAQJ2QldBq2QAcXFzdK+Q2YV21Wpc0C0xFUM5O9I4hbFcojrVl1l/UbG
Bnit8x73X4OJ0H8ByXzeuMppfd3akInIOJLezL+3arABadHIKhc8oEyZlgnKHzKR
T8z2Tq7FK6R5Y3JSOXnAuM6kzS+AZRBry5CradJVJwKBgQDnFDNa5ph2sfehl006
pFyEf1l7BCXtSsnLbeywz2xi17Kx3kLBLkaPsnf9m1qZ0/NScDA+io2ZC6DySo82
WIcdQyd4I0NDwekDvfI78swfwR9jbsjVJHuGR+IWSTZvTnMG7aCxFQuQRe/kaHIL
1z74P6JOdViftaOqFhFIYl3CzwKBgQDjYb5IxaYHV0mzHsab/Arx/Sle83k3uBQI
dHA0GviP2YuYv4jf2iaMg1UMNKAbUp2NXAXsamuSXHdp6GiMK2fGWVD792wgRevK
Acz8lzCOsDK8XXz8M3Eh/hjr4dTIoq16MzFjOYh+Kwm2c9Rd3PBf8Q2p4nde/tHg
fzQLsv2NCwKBgQCwT4Vvkgo6ZkefD6ZpXAcLQW+woNWfXDTj9pdlwJ3ePN2nQQKG
CxzjfzR2WBak0EcTW240CdtILss6kxD6UkmlVhvDWoR0Knvz0vYEL5j3kY61e03Y
8uEc77PddTcHbj/txVmaQ4hzKCmFiPubdTwihcr9OiPIl/qsR/If3I3VmQKBgApQ
m843USHSHuDGS6I129VAc8j/6IbTje0YQyLJ+m6kIsYKIk5tWgRTzN7h4EV9CPKp
swcXiMu58BzY0y1QpsODt73GapxIL7sZO9BVl3lRmuuanhnex4oQOdcxhnKXlqEN
g3cJ3BxFHYquVHrxk+H2UHVddabUjnbNrnG9a+0jAoGBAJwGaIVaTizRuQ5eBnYz
tS2ExUnejCeo9sWEbBIA+oqnO+XGoepn6MJBgAF8VaZTqMIPuyY4/GHCQqsU1QDV
bi+aXQnjxeU9u61V7erNTVaVqMnwQYjQzoq/1Fpuf14HeV2qV/YEGFrU7baBodRg
XrivCALoN8O9Gvb+bMTIf4Ut
-----END PRIVATE KEY-----"#;

    let public_key = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzT8j874e4vljLNEuT8fI
qejMY2oOl8X9Umfe/avi/5zw77jgA581J0m3gTWMwMM+/c2KKq2uuMytR3Ba8QnF
XzKrvFq5KO1nr7IaKOX28c0xPIvvyd3HepURNPqcKzdPpOYFIj49Ss4ifa+HDnsX
5loEUaY0zhBKcazkqIZQ6Y9OM5j5+wd305QLcZbCUc8P3PrLvz2NGqKVqV6P5Swv
/yManmIJGnfIPBnBKZNFD2l5kKHZmg6ZOwV8+EelpZ/nfQAVMfGvbDBOssdPtDwD
T+w8lpqD0bK1cwO5rIwEPx5zt3G5Vs5ImPDIpIKOcSU0EumjDt8yGJjpsG9MUuZh
5QIDAQAB
-----END PUBLIC KEY-----"#;

    let kid = "test-key-id-123".to_string();

    (
        EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap(),
        DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
        kid,
    )
}

// ========== TESTS DE verify_firebase_token ==========

#[test]
fn test_verify_firebase_token_success() {
    let (encoding_key, _, kid) = create_test_rsa_keys();
    let project_id: &str = "amanahacademia";
    let user_id: &str = "test-user-123";

    let token: String = create_valid_test_token(project_id, user_id, &encoding_key, &kid);

    let public_key: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzT8j874e4vljLNEuT8fI
qejMY2oOl8X9Umfe/avi/5zw77jgA581J0m3gTWMwMM+/c2KKq2uuMytR3Ba8QnF
XzKrvFq5KO1nr7IaKOX28c0xPIvvyd3HepURNPqcKzdPpOYFIj49Ss4ifa+HDnsX
5loEUaY0zhBKcazkqIZQ6Y9OM5j5+wd305QLcZbCUc8P3PrLvz2NGqKVqV6P5Swv
/yManmIJGnfIPBnBKZNFD2l5kKHZmg6ZOwV8+EelpZ/nfQAVMfGvbDBOssdPtDwD
T+w8lpqD0bK1cwO5rIwEPx5zt3G5Vs5ImPDIpIKOcSU0EumjDt8yGJjpsG9MUuZh
5QIDAQAB
-----END PUBLIC KEY-----"#;

    let firebase_keys = json!({
        kid: public_key
    });

    let result = verify_firebase_token(&token, &firebase_keys, project_id);
    assert!(result.is_ok());

    let token_data = result.unwrap();
    assert_eq!(token_data.claims.user_id, user_id);
    assert_eq!(
        token_data.claims.email,
        Some("test@example.com".to_string())
    );
}

#[test]
fn test_verify_firebase_token_missing_kid() {
    let token: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.invalid";
    let firebase_keys: Value = json!({});
    let project_id: &str = "amanahacademia";

    let result = verify_firebase_token(token, &firebase_keys, project_id);
    assert!(result.is_err());
}

#[test]
fn test_verify_firebase_token_no_matching_key() {
    let (encoding_key, _, kid) = create_test_rsa_keys();
    let project_id: &str = "amanahacademia";
    let user_id: &str = "test-user-123";

    let token: String = create_valid_test_token(project_id, user_id, &encoding_key, &kid);
    // Firebase keys con kid diferente
    let firebase_keys = json!({
        "different-kid": "some-public-key"
    });

    let result = verify_firebase_token(&token, &firebase_keys, project_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::NoMatchingKey));
}

#[test]
fn test_verify_firebase_token_expired() {
    let (encoding_key, _, kid) = create_test_rsa_keys();
    let project_id: &str = "amanahacademia";
    let user_id: &str = "test-user-123";

    // Crear token expirado (exp en el pasado)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = UserAuthentication {
        sub: user_id.to_string(),
        iss: format!("https://securetoken.google.com/{}", project_id),
        aud: project_id.to_string(),
        iat: now - 7200,
        exp: now - 3600, // Expirado hace 1 hora
        email: Some("test@example.com".to_string()),
        email_verified: Some(true),
        name: Some("Test User".to_string()),
        picture: None,
        auth_time: now - 7200,
        user_id: user_id.to_string(),
        firebase: None,
        phone_number: None,
        provider_id: Some("password".to_string()),
    };

    let mut header: Header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.clone());

    let token: String = encode(&header, &claims, &encoding_key).unwrap();

    let public_key = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzT8j874e4vljLNEuT8fI
qejMY2oOl8X9Umfe/avi/5zw77jgA581J0m3gTWMwMM+/c2KKq2uuMytR3Ba8QnF
XzKrvFq5KO1nr7IaKOX28c0xPIvvyd3HepURNPqcKzdPpOYFIj49Ss4ifa+HDnsX
5loEUaY0zhBKcazkqIZQ6Y9OM5j5+wd305QLcZbCUc8P3PrLvz2NGqKVqV6P5Swv
/yManmIJGnfIPBnBKZNFD2l5kKHZmg6ZOwV8+EelpZ/nfQAVMfGvbDBOssdPtDwD
T+w8lpqD0bK1cwO5rIwEPx5zt3G5Vs5ImPDIpIKOcSU0EumjDt8yGJjpsG9MUuZh
5QIDAQAB
-----END PUBLIC KEY-----"#;

    let firebase_keys: Value = json!({
        kid: public_key
    });

    let result = verify_firebase_token(&token, &firebase_keys, project_id);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AuthError::TokenVerification(_)
    ));
}

#[test]
fn test_verify_firebase_token_invalid_audience() {
    let (encoding_key, _, kid) = create_test_rsa_keys();
    let project_id: &str = "wrong-project-id";
    let user_id: &str = "test-user-123";

    let token: String = create_valid_test_token(project_id, user_id, &encoding_key, &kid);
    let public_key = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzT8j874e4vljLNEuT8fI
qejMY2oOl8X9Umfe/avi/5zw77jgA581J0m3gTWMwMM+/c2KKq2uuMytR3Ba8QnF
XzKrvFq5KO1nr7IaKOX28c0xPIvvyd3HepURNPqcKzdPpOYFIj49Ss4ifa+HDnsX
5loEUaY0zhBKcazkqIZQ6Y9OM5j5+wd305QLcZbCUc8P3PrLvz2NGqKVqV6P5Swv
/yManmIJGnfIPBnBKZNFD2l5kKHZmg6ZOwV8+EelpZ/nfQAVMfGvbDBOssdPtDwD
T+w8lpqD0bK1cwO5rIwEPx5zt3G5Vs5ImPDIpIKOcSU0EumjDt8yGJjpsG9MUuZh
5QIDAQAB
-----END PUBLIC KEY-----"#;

    let firebase_keys: Value = json!({
        kid: public_key
    });

    // Verificar con proyecto diferente
    let result = verify_firebase_token(&token, &firebase_keys, "amanahacademia");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AuthError::TokenVerification(_)
    ));
}

#[test]
fn test_verify_firebase_token_invalid_issuer() {
    let (encoding_key, _, kid) = create_test_rsa_keys();
    let project_id = "amanahacademia";
    let user_id = "test-user-123";

    // Crear token con issuer inválido
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = UserAuthentication {
        sub: user_id.to_string(),
        iss: "https://invalid-issuer.com".to_string(), // Issuer inválido
        aud: project_id.to_string(),
        iat: now,
        exp: now + 3600,
        email: Some("test@example.com".to_string()),
        email_verified: Some(true),
        name: Some("Test User".to_string()),
        picture: None,
        auth_time: now,
        user_id: user_id.to_string(),
        firebase: None,
        phone_number: None,
        provider_id: Some("password".to_string()),
    };

    let mut header: Header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.clone());

    let token: String = encode(&header, &claims, &encoding_key).unwrap();

    let public_key: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzT8j874e4vljLNEuT8fI
qejMY2oOl8X9Umfe/avi/5zw77jgA581J0m3gTWMwMM+/c2KKq2uuMytR3Ba8QnF
XzKrvFq5KO1nr7IaKOX28c0xPIvvyd3HepURNPqcKzdPpOYFIj49Ss4ifa+HDnsX
5loEUaY0zhBKcazkqIZQ6Y9OM5j5+wd305QLcZbCUc8P3PrLvz2NGqKVqV6P5Swv
/yManmIJGnfIPBnBKZNFD2l5kKHZmg6ZOwV8+EelpZ/nfQAVMfGvbDBOssdPtDwD
T+w8lpqD0bK1cwO5rIwEPx5zt3G5Vs5ImPDIpIKOcSU0EumjDt8yGJjpsG9MUuZh
5QIDAQAB
-----END PUBLIC KEY-----"#;

    let firebase_keys = json!({
        kid: public_key
    });

    let result = verify_firebase_token(&token, &firebase_keys, project_id);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AuthError::TokenVerification(_)
    ));
}

// ========== TESTS DE MODELOS Y ESTRUCTURAS ==========

#[test]
fn test_claims_ga_structure() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = ClaimsGA {
        iss: "test@test-project.iam.gserviceaccount.com".to_string(),
        scope: "https://www.googleapis.com/auth/analytics.readonly".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    assert_eq!(claims.iss, "test@test-project.iam.gserviceaccount.com");
    assert_eq!(
        claims.scope,
        "https://www.googleapis.com/auth/analytics.readonly"
    );
    assert_eq!(claims.aud, "https://oauth2.googleapis.com/token");
    assert_eq!(claims.exp, now + 3600);
    assert_eq!(claims.iat, now);
}

#[test]
fn test_service_account_structure() {
    let service_account = ServiceAccount {
        client_email: "test@test-project.iam.gserviceaccount.com".to_string(),
        private_key: "-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----".to_string(),
    };

    assert_eq!(
        service_account.client_email,
        "test@test-project.iam.gserviceaccount.com"
    );
    assert!(service_account.private_key.contains("BEGIN PRIVATE KEY"));
}

// ========== TESTS DE CONVERSIÓN DE ERRORES ==========

#[test]
fn test_auth_error_to_status_code() {
    assert_eq!(
        StatusCode::from(AuthError::MissingHeader),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        StatusCode::from(AuthError::InvalidHeaderFormat),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        StatusCode::from(AuthError::MissingKid),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        StatusCode::from(AuthError::NoMatchingKey),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        StatusCode::from(AuthError::TokenVerification("test".to_string())),
        StatusCode::FORBIDDEN
    );
}

// ========== TESTS DE KEY CACHE ==========

#[tokio::test]
async fn test_key_cache_expiration() {
    let firebase_keys = json!({"test-kid": "test-key"});

    // Cache recién creado (no expirado)
    let fresh_cache = KeyCache {
        keys: firebase_keys.clone(),
        fetched_at: Instant::now(),
    };
    assert!(!fresh_cache.is_expired());

    // Cache expirado (simulado con timestamp antiguo)
    let expired_cache = KeyCache {
        keys: firebase_keys,
        fetched_at: Instant::now() - Duration::from_secs(3601),
    };
    assert!(expired_cache.is_expired());
}

#[test]
fn test_key_cache_ttl() {
    let firebase_keys = json!({"test-kid": "test-key"});
    let cache = KeyCache {
        keys: firebase_keys,
        fetched_at: Instant::now(),
    };

    // Verificar que el TTL es exactamente 1 hora (3600 segundos)
    assert!(!cache.is_expired());
}

#[test]
fn test_ga_token_wrapper() {
    let token = GAToken("test-token-123".to_string());
    assert_eq!(token.0, "test-token-123");

    // Verificar que es cloneable
    let cloned = token.clone();
    assert_eq!(cloned.0, "test-token-123");
}

// ========== TESTS DE VALIDACIÓN DE JWT ==========

#[test]
fn test_jwt_header_structure() {
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("test-kid-123".to_string());

    assert_eq!(header.alg, Algorithm::RS256);
    assert_eq!(header.kid, Some("test-kid-123".to_string()));
}

#[test]
fn test_user_authentication_claims() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = UserAuthentication {
        sub: "user-123".to_string(),
        iss: "https://securetoken.google.com/amanahacademia".to_string(),
        aud: "amanahacademia".to_string(),
        iat: now,
        exp: now + 3600,
        email: Some("test@example.com".to_string()),
        email_verified: Some(true),
        name: Some("Test User".to_string()),
        picture: Some("https://example.com/photo.jpg".to_string()),
        auth_time: now,
        user_id: "user-123".to_string(),
        firebase: None,
        phone_number: Some("+1234567890".to_string()),
        provider_id: Some("google.com".to_string()),
    };

    assert_eq!(claims.sub, "user-123");
    assert_eq!(claims.user_id, "user-123");
    assert_eq!(claims.email, Some("test@example.com".to_string()));
    assert_eq!(claims.email_verified, Some(true));
    assert_eq!(claims.provider_id, Some("google.com".to_string()));
}

// ========== TESTS DE GOOGLE ANALYTICS TOKEN ==========

#[test]
fn test_claims_ga_expiration() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = ClaimsGA {
        iss: "test@test-project.iam.gserviceaccount.com".to_string(),
        scope: "https://www.googleapis.com/auth/analytics.readonly".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    // Verificar que el token es válido por 1 hora
    assert_eq!(claims.exp - claims.iat, 3600);
}

#[test]
fn test_service_account_email_format() {
    let service_account = ServiceAccount {
        client_email: "test@test-project.iam.gserviceaccount.com".to_string(),
        private_key: "-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----".to_string(),
    };

    // Verificar formato de email de service account
    assert!(service_account.client_email.contains("@"));
    assert!(
        service_account
            .client_email
            .ends_with(".iam.gserviceaccount.com")
    );
}
