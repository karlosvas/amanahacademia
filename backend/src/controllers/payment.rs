use {
    crate::{
        models::stripe::{
            CurrencyMap, PayloadCreacteProduct, PaymentPayload, PaymentResponse, PricePayload,
        },
        state::AppState,
    },
    axum::{
        Json, debug_handler,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, str::FromStr, sync::Arc},
    stripe::{
        CreatePaymentIntent, CreateProduct, CreateProductDefaultPriceData,
        CreateProductDefaultPriceDataCurrencyOptions, Currency, PaymentIntent, PaymentIntentStatus,
        Product, ProductType,
    },
};

// Comprar precios genericos
#[debug_handler]
pub async fn generic_payment(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PaymentPayload>,
) -> impl IntoResponse {
    // Validar el monto mínimo (ejemplo: $5.00 USD = 500 centavos)
    if payload.amount < 500 {
        return Json(json!({
            "succes": false,
            "message": Some("El monto mínimo es 5.00€".to_string()),
        }));
    }

    // Crear el PaymentIntent
    let payment_intent: CreatePaymentIntent = CreatePaymentIntent {
        amount: payload.amount,
        currency: Currency::from_str(&payload.currency.to_string()).unwrap_or(Currency::EUR),
        application_fee_amount: None,
        automatic_payment_methods: None,
        capture_method: None,
        confirm: Some(true),
        confirmation_method: None,
        customer: None,
        description: None,
        error_on_requires_action: None,
        expand: &[],
        mandate: None,
        mandate_data: None,
        metadata: None,
        off_session: None,
        on_behalf_of: None,
        payment_method: Some(stripe::PaymentMethodId::from_str(&payload.payment_method).unwrap()),
        payment_method_configuration: None,
        payment_method_data: None,
        payment_method_options: None,
        payment_method_types: None,
        radar_options: None,
        receipt_email: None,
        return_url: Some("https://amanahacademia.com"),
        setup_future_usage: None,
        shipping: None,
        statement_descriptor: None,
        statement_descriptor_suffix: None,
        transfer_data: None,
        transfer_group: None,
        use_stripe_sdk: None,
    };

    match PaymentIntent::create(&state.stripe_client, payment_intent).await {
        Ok(payment_intent) => match payment_intent.status {
            PaymentIntentStatus::Succeeded => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "succes".to_string(),
                error: Some("Payment successful".to_string()),
            })),
            PaymentIntentStatus::RequiresAction => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_action".to_string(),
                error: Some("Requires additional authentication".to_string()),
            })),
            PaymentIntentStatus::RequiresPaymentMethod => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_payment_method".to_string(),
                error: Some("Invalid payment method".to_string()),
            })),
            PaymentIntentStatus::Canceled => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "canceled".to_string(),
                error: Some("Payment canceled".to_string()),
            })),
            PaymentIntentStatus::Processing => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "processing".to_string(),
                error: Some("Payment processing".to_string()),
            })),
            _ => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: payment_intent.status.to_string(),
                error: None,
            })),
        },
        Err(_) => Json(json!(PaymentResponse {
            client_secret: None,
            status: "error".to_string(),
            error: Some("Error processing payment".to_string()),
        })),
    }
}

// Comprar clase individual
pub async fn basic_class_payment(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PaymentPayload>,
) -> impl IntoResponse {
    // Validar el monto mínimo (ejemplo: $5.00 USD = 500 centavos)
    if payload.amount < 500 {
        return Json(json!({
            "succes": false,
            "message": Some("El monto mínimo es 5.00€".to_string()),
        }));
    }

    // Crear el PaymentIntent
    let payment_intent: CreatePaymentIntent = CreatePaymentIntent {
        amount: payload.amount,
        currency: Currency::from_str(&payload.currency.to_string()).unwrap_or(Currency::EUR),
        application_fee_amount: None,
        automatic_payment_methods: None,
        capture_method: None,
        confirm: Some(true),
        confirmation_method: None,
        customer: None,
        description: None,
        error_on_requires_action: None,
        expand: &[],
        mandate: None,
        mandate_data: None,
        metadata: None,
        off_session: None,
        on_behalf_of: None,
        payment_method: Some(stripe::PaymentMethodId::from_str(&payload.payment_method).unwrap()),
        payment_method_configuration: None,
        payment_method_data: None,
        payment_method_options: None,
        payment_method_types: None,
        radar_options: None,
        receipt_email: None,
        return_url: Some("https://amanahacademia.com"),
        setup_future_usage: None,
        shipping: None,
        statement_descriptor: None,
        statement_descriptor_suffix: None,
        transfer_data: None,
        transfer_group: None,
        use_stripe_sdk: None,
    };

    match PaymentIntent::create(&state.stripe_client, payment_intent).await {
        Ok(payment_intent) => match payment_intent.status {
            PaymentIntentStatus::Succeeded => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "succes".to_string(),
                error: Some("Payment successful".to_string()),
            })),
            PaymentIntentStatus::RequiresAction => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_action".to_string(),
                error: Some("Requires additional authentication".to_string()),
            })),
            PaymentIntentStatus::RequiresPaymentMethod => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_payment_method".to_string(),
                error: Some("Invalid payment method".to_string()),
            })),
            PaymentIntentStatus::Canceled => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "canceled".to_string(),
                error: Some("Payment canceled".to_string()),
            })),
            PaymentIntentStatus::Processing => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "processing".to_string(),
                error: Some("Payment processing".to_string()),
            })),
            _ => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: payment_intent.status.to_string(),
                error: None,
            })),
        },
        Err(_) => Json(json!(PaymentResponse {
            client_secret: None,
            status: "error".to_string(),
            error: Some("Error processing payment".to_string()),
        })),
    }
}

// Consultar un payment para saber su estado actual
pub async fn get_payment_status(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Cancelar un payment
pub async fn cancel_payment(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Devolver el pago
pub async fn refund_payment(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Consultar el historial de pagos del usuario
pub async fn get_payment_history(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Webhook para recibir eventos de Stripe
pub async fn webhook_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

// Crear un producto
#[debug_handler]
pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PayloadCreacteProduct>,
) -> impl IntoResponse {
    // Monedas diferentes a la original EURO
    let mut currency_opts: CurrencyMap<CreateProductDefaultPriceDataCurrencyOptions> =
        HashMap::new();
    insert_options_by_country(&mut currency_opts, &payload.price);

    // Creeamos un precio
    let pricing: CreateProductDefaultPriceData = CreateProductDefaultPriceData {
        // Obligatorios
        currency: match Currency::from_str(&payload.price.currency.to_string()) {
            Ok(currency) => {
                if currency != Currency::EUR {
                    return Json(json!({
                        "error": "Invalid currency, must be EUR"
                    }));
                }
                currency
            }
            Err(_) => {
                return Json(json!({
                    "error": "Invalid currency"
                }));
            }
        },
        unit_amount: Some(payload.price.unit_amount),
        // Si es recurrente
        recurring: match payload.price.recurring {
            Some(recurring) => Some(recurring),
            None => None, // Si no es recurrente no pasa nada
        },
        // Opcionales
        currency_options: Some(currency_opts),
        tax_behavior: None,
        unit_amount_decimal: None,
    };

    // Creamos un nuevo producto con todos los datos que sabemos tanto del producto como del nuevo precio
    let new_product: CreateProduct = CreateProduct {
        // Obligatorios
        name: &payload.product.name,
        description: Some(&payload.product.description),
        images: None,
        metadata: None,
        active: Some(true),
        shippable: Some(false),
        unit_label: None, // ??
        tax_code: None,
        url: Some("https://amanahacademia.com/pricing"),
        expand: &[],
        default_price_data: Some(pricing),
        features: None,
        id: None,
        package_dimensions: None,
        statement_descriptor: None,
        type_: Some(ProductType::Good),
    };

    match Product::create(&state.stripe_client, new_product).await {
        Ok(product) => Json(json!({ "success": true, "product": product })),
        Err(_err) => Json(json!({
            "success": false,
            "error": "Error creating product"
        })),
    }
}

// Añadir otras monedas
fn insert_options_by_country(
    currency_opts: &mut CurrencyMap<CreateProductDefaultPriceDataCurrencyOptions>,
    base_price: &PricePayload,
) {
    let amount_eur: i64 = base_price.unit_amount;

    // USD (ejemplo: 1 EUR = 1.1 USD)
    let amount_usd: i64 = ((amount_eur as f64) * 1.1).round() as i64;
    currency_opts.insert(
        Currency::USD,
        CreateProductDefaultPriceDataCurrencyOptions {
            unit_amount: Some(amount_usd),
            unit_amount_decimal: None,
            tax_behavior: None,
            custom_unit_amount: None,
            tiers: None,
        },
    );

    // SAR (ejemplo: 1 EUR = 4.1 SAR)
    let amount_sar = ((amount_eur as f64) * 4.1).round() as i64;
    currency_opts.insert(
        Currency::SAR,
        CreateProductDefaultPriceDataCurrencyOptions {
            unit_amount: Some(amount_sar),
            unit_amount_decimal: None,
            tax_behavior: None,
            custom_unit_amount: None,
            tiers: None,
        },
    );
}

pub async fn get_all_products() {}
