use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    stripe::{CreateProductDefaultPriceDataRecurring, Currency, PaymentIntent},
};

/// Payload para crear un PaymentIntent (pago único)
#[derive(Debug, Deserialize)]
pub struct PaymentPayload {
    /// Cantidad en la unidad más pequeña de la moneda (céntimos para USD/EUR)
    pub amount: i64,
    /// Código ISO de moneda en minúsculas (ej: "usd", "eur")
    pub currency: String,
}

/// Respuesta tras crear o consultar un PaymentIntent
#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    /// Secret usado por Stripe.js en el frontend para confirmar el pago
    pub client_secret: Option<String>,
    /// Estado del pago: "requires_payment_method", "succeeded", "canceled", etc.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
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

/// Relación entre un evento de Cal.com y un producto/precio de Stripe
#[derive(Debug, Deserialize, Serialize)]
pub struct RelationalCalStripe {
    /// ID del evento en Cal.com
    pub cal_id: String,
    /// ID del producto o precio en Stripe (price_xxx o prod_xxx)
    pub stripe_id: String,
}

/// Relación entre un evento de Cal.com y un producto/precio de Stripe
#[derive(Debug, Deserialize, Serialize)]
pub struct StripeRelation {
    pub stripe_id: String,
}

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
