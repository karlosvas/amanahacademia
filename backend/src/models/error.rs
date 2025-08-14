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
