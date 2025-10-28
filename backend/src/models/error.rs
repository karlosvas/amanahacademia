use {
    crate::models::{metrics::GAErrorResponse, response::ResponseAPI},
    axum::{Json, http::StatusCode},
};

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

/// Errores relacionados con las metricas de Google Analitics
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Google Analytics API error: {message} ({status})")]
    Api { message: String, status: String },

    #[error("Google Analytics returned error: {0}")]
    ApiText(String),

    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<MetricsError> for (StatusCode, Json<ResponseAPI<()>>) {
    fn from(err: MetricsError) -> Self {
        let (status, message) = match &err {
            MetricsError::Network(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch metrics: {}", err),
            ),
            MetricsError::Api { message, status } => (
                StatusCode::BAD_REQUEST,
                format!("Google Analytics API error: {} ({})", message, status),
            ),
            MetricsError::ApiText(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
            MetricsError::Parse(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
        };

        (status, Json(ResponseAPI::error(message)))
    }
}

impl From<GAErrorResponse> for MetricsError {
    fn from(ga_err: GAErrorResponse) -> Self {
        MetricsError::Api {
            message: ga_err.error.message,
            status: ga_err.error.status,
        }
    }
}
