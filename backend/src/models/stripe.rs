use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    stripe::{CreateProductDefaultPriceDataRecurring, Currency, PaymentIntent},
};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PriceTable {
    pub individual_standard: f64,
    pub individual_conversation: f64,
    pub group: f64,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutPayload {
    pub booking_uid: Option<String>,
    pub class_type: Option<String>,
    pub weeks: Option<u32>,
    pub prices: PriceTable,
}

#[derive(Debug, Serialize)]
pub struct CheckoutResponse {
    pub client_secret: String,
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingPayment {
    pub email: String,
    pub amount: i64,
    pub class_type: String,
    pub created: i64,
}

/// Payload para crear un producto en Stripe
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPayload {
    pub name: String,
    pub description: String,
    /// URLs de imágenes del producto (máximo recomendado: 8)
    pub images: Vec<String>,
    /// Pares clave-valor arbitrarios para almacenar info adicional (máx 50 keys)
    pub metadata: HashMap<String, String>,
    /// Si false, el producto no estará disponible para nuevas compras
    pub active: bool,
}

/// Payload para crear el precio de un producto
#[derive(Debug, Serialize, Deserialize)]
pub struct PricePayload {
    /// Código ISO de moneda en minúsculas
    pub currency: String,
    /// Precio en la unidad más pequeña (céntimos). Ej: 1000 = $10.00
    pub unit_amount: i64,
    /// Si es Some, el precio será recurrente (suscripción). Si None, pago único.
    pub recurring: Option<CreateProductDefaultPriceDataRecurring>,
}

/// Payload completo para crear un producto con su precio en una sola operación
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadCreacteProduct {
    pub product: ProductPayload,
    pub price: PricePayload,
}

/// HashMap indexado por tipo de moneda (útil para precios multi-moneda)
pub type CurrencyMap<T> = HashMap<Currency, T>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntentSimplified {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub created: i64,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
    pub payment_method_types: Vec<String>,
}

impl From<PaymentIntent> for PaymentIntentSimplified {
    fn from(pi: PaymentIntent) -> Self {
        Self {
            id: pi.id.to_string(),
            amount: pi.amount,
            currency: pi.currency.to_string(),
            status: pi.status.to_string(),
            created: pi.created,
            description: pi.description,
            metadata: pi.metadata,
            payment_method_types: pi
                .payment_method_types
                .into_iter()
                .map(|t| t.to_string())
                .collect(),
        }
    }
}
