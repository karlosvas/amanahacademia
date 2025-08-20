use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use stripe::{CreateProductDefaultPriceDataRecurring, Currency};

/// Estructura para el payload de creación de un PaymentIntent
#[derive(Debug, Deserialize)]
pub struct PaymentPayload {
    pub amount: i64,
    pub currency: String,
    pub payment_method: String,
}

/// Estructura para la respuesta de un PaymentIntent
#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub client_secret: Option<String>,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPayload {
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub active: bool,
    pub caption: String,
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
