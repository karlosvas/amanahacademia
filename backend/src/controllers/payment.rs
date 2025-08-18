use {
    crate::{
        models::stripe::{PaymentPayload, PaymentResponse},
        state::AppState,
    },
    axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    serde_json::json,
    std::{str::FromStr, sync::Arc},
    stripe::{CreatePaymentIntent, Currency, PaymentIntent, PaymentIntentStatus},
};

// Comprar precios basicos
pub async fn basic_class(
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
        payment_method: None,
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
                status: "succeeded".to_string(),
                error: Some("Pago exitoso".to_string()),
            })),
            PaymentIntentStatus::RequiresAction => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_action".to_string(),
                error: Some("Requiere autenticación adicional".to_string()),
            })),
            PaymentIntentStatus::RequiresPaymentMethod => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "requires_payment_method".to_string(),
                error: Some("Método de pago inválido".to_string()),
            })),
            PaymentIntentStatus::Canceled => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "canceled".to_string(),
                error: Some("Pago cancelado".to_string()),
            })),
            PaymentIntentStatus::Processing => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: "processing".to_string(),
                error: Some("Pago en proceso".to_string()),
            })),
            _ => Json(json!(PaymentResponse {
                client_secret: payment_intent.client_secret,
                status: payment_intent.status.to_string(),
                error: None,
            })),
        },
        Err(e) => {
            eprintln!("Error creating payment intent: {:?}", e);
            Json(json!(PaymentResponse {
                client_secret: None,
                status: "error".to_string(),
                error: Some("Error procesando el pago".to_string()),
            }))
        }
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
