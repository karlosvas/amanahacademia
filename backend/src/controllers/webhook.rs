use {
    crate::models::{
        response::ResponseAPI,
        stripe::RelationalCalStripe,
        webhook::{CalWebhookEvent, RefundResponse},
    },
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
    stripe::{CreateRefund, PaymentIntentId, Refund},
};

use crate::state::AppState;

// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn handle_cal_webhook(
    State(state): State<Arc<AppState>>,
    Json(event): Json<CalWebhookEvent>,
) -> impl IntoResponse {
    tracing::info!("📨 Webhook received: {}", event.trigger_event);
    tracing::debug!("Full payload: {:?}", event);

    // Si el evento es cancelación, procesamos refund
    if event.trigger_event == "BOOKING_CANCELLED" {
        tracing::info!(
            "Booking cancelled: {} - Reason: {:?}",
            event.payload.uid,
            event.payload.cancellation_reason
        );

        let booking_id = &event.payload.uid;

        let url_firebase_db_relationship = format!(
            "{}/relation_cal_stripe/{}.json",
            state.firebase.firebase_database_url, booking_id
        );

        // Obtener relación booking <-> Stripe desde Firebase
        let stripe_id: String = match state
            .firebase_client
            .get(&url_firebase_db_relationship)
            .send()
            .await
        {
            Ok(res) => match res.json::<Option<RelationalCalStripe>>().await {
                Ok(Some(val)) => val.stripe_id,
                Ok(None) => {
                    tracing::error!(
                        "No se encontró relación de Firebase para booking_id: {}",
                        booking_id
                    );
                    return (
                        StatusCode::OK,
                        Json(ResponseAPI::<()>::error("No Stripe ID found".to_string())),
                    )
                        .into_response();
                }
                Err(e) => {
                    tracing::error!("Error parseando respuesta de Firebase: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<()>::error("Firebase parse error".to_string())),
                    )
                        .into_response();
                }
            },
            Err(e) => {
                tracing::error!("Error obteniendo relación de Firebase: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Firebase request error".to_string(),
                    )),
                )
                    .into_response();
            }
        };

        // Convertir a PaymentIntentId
        let pi_id: PaymentIntentId = match stripe_id.parse() {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Invalid PaymentIntent ID: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error(
                        "Invalid PaymentIntent ID".to_string(),
                    )),
                )
                    .into_response();
            }
        };

        // Crear refund en Stripe
        match Refund::create(
            &state.stripe_client,
            CreateRefund {
                payment_intent: Some(pi_id),
                ..Default::default()
            },
        )
        .await
        {
            Ok(refund) => {
                tracing::info!("Reembolso creado en Stripe: {:?}", refund);

                let response = RefundResponse {
                    id: refund.id.to_string(),
                    amount: refund.amount,
                    currency: refund.currency.to_string(),
                    status: refund.status.clone(),
                    created: refund.created,
                };
                return (
                    StatusCode::OK,
                    Json(ResponseAPI::<RefundResponse>::success(
                        "Refund created successfully".to_string(),
                        response,
                    )),
                )
                    .into_response();
            }
            Err(e) => {
                tracing::error!("Error creando reembolso: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Failed to create refund".to_string(),
                    )),
                )
                    .into_response();
            }
        }
    }

    (
        StatusCode::NOT_ACCEPTABLE,
        Json(ResponseAPI::<()>::error(
            "Event received but not processed".to_string(),
        )),
    )
        .into_response()
}
