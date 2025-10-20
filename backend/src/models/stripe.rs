use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use stripe::{CreateProductDefaultPriceDataRecurring, Currency};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPayload {
    pub name: String,        // Nombre del producto (ejemplo: "Curso de Inglés").
    pub description: String, // Descripción del producto (ejemplo: "Un curso de inglés para principiantes").
    pub images: Vec<String>, // Imágenes del producto (ejemplo: ["http://example.com/image.png"]).
    pub metadata: HashMap<String, String>, // Metadatos del producto (ejemplo: {"category": "digital", "tags": ["tag1", "tag2"]}).
    pub active: bool,                      // Indica si el producto está activo o no.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricePayload {
    pub currency: String,
    pub unit_amount: i64,
    pub recurring: Option<CreateProductDefaultPriceDataRecurring>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadCreacteProduct {
    pub product: ProductPayload,
    pub price: PricePayload,
}

pub type CurrencyMap<T> = HashMap<Currency, T>;

// #[derive(Debug, Deserialize)]
// pub struct BookingPaymentPayload {
//     pub amount: i64,
//     pub currency: String,
//     pub payment_method: String,
//     // Datos del booking
//     pub event_type_id: i64,
//     pub start_time: String, // ISO 8601
//     pub attendee_name: String,
//     pub attendee_email: String,
//     pub attendee_timezone: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub attendee_phone: Option<String>,
// }

#[derive(Debug, Deserialize)]
pub struct RelationalCalStripe {
    pub cal_id: String,
    pub stripe_id: String,
}
