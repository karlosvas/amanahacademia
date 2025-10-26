use axum::http::StatusCode;

/// Errores relacionados con la autenticación de Firebase
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    MissingHeader,
    #[error("Invalid authorization header format")]
    InvalidHeaderFormat,
    #[error("Token verification failed: {0}")]
    TokenVerification(String),
    #[error("Missing kid in token header")]
    MissingKid,
    #[error("No matching public key found")]
    NoMatchingKey,
    #[error("Invalid key format")]
    InvalidKeyFormat,
}

impl From<AuthError> for StatusCode {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::MissingHeader | AuthError::InvalidHeaderFormat => StatusCode::UNAUTHORIZED,
            AuthError::TokenVerification(_)
            | AuthError::MissingKid
            | AuthError::NoMatchingKey
            | AuthError::InvalidKeyFormat => StatusCode::FORBIDDEN,
        }
    }
}
