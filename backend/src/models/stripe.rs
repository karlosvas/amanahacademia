use serde::{Deserialize, Serialize};

/// Estructura para el payload de creación de un PaymentIntent
#[derive(Debug, Deserialize)]
pub struct PaymentPayload {
    pub amount: i64,
    pub currency: String,
}

/// Estructura para la respuesta de un PaymentIntent
#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub client_secret: Option<String>,
    pub status: String,
    pub error: Option<String>,
}
