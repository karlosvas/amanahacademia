use {
    crate::models::webhook::CalWebhookEvent,
    axum::{Json, extract::State, http::StatusCode},
    std::sync::Arc,
};

use crate::state::AppState;

// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn handle_cal_webhook(
    State(state): State<Arc<AppState>>,
    Json(event): Json<CalWebhookEvent>,
) -> StatusCode {
    tracing::info!("📨 Webhook received: {}", event.trigger_event);
    tracing::debug!("Full payload: {:?}", event);

    tracing::debug!("Full cancellation payload: {:?}", event);
    if event.trigger_event == "BOOKING_CANCELLED" {
        tracing::info!(
            "❌ Booking cancelled: {} - Reason: {:?}",
            event.payload.uid,
            event.payload.cancellation_reason
        );
    }

    // SIEMPRE retornar 200 OK
    StatusCode::OK
}

// backend/src/controllers/payments.rs - Actualizar webhook_handler existente
// pub async fn webhook_handler(
//     State(state): State<Arc<AppState>>,
//     headers: axum::http::HeaderMap,
//     body: String,
// ) -> impl IntoResponse {
//     let signature = match headers.get("stripe-signature") {
//         Some(sig) => sig.to_str().unwrap_or(""),
//         None => {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({"error": "Missing stripe-signature header"})),
//             );
//         }
//     };
//     let webhook_secret =
//         std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_...".to_string());
//     // Verificar firma del webhook
//     let event = match Webhook::construct_event(&body, signature, &webhook_secret) {
//         Ok(event) => event,
//         Err(e) => {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({"error": format!("Webhook signature verification failed: {}", e)})),
//             );
//         }
//     };
//     // Procesar evento
//     match event.type_ {
//         EventType::PaymentIntentSucceeded => {
//             if let EventObject::PaymentIntent(payment_intent) = event.data.object {
//                 tracing::info!("PaymentIntent succeeded: {}", payment_intent.id);
//                 // Obtener pending booking de DB usando payment_intent.id
//                 // let pending_booking = get_pending_booking(&state.db, &payment_intent.id).await?;
//                 // Crear booking en Cal.com
//                 // let booking = create_cal_booking_from_pending(&state, &pending_booking).await?;
//                 // Marcar como completado en DB
//                 // mark_booking_completed(&state.db, &payment_intent.id, &booking.uid).await?;
//             }
//         }
//         EventType::PaymentIntentPaymentFailed => {
//             if let EventObject::PaymentIntent(payment_intent) = event.data.object {
//                 tracing::warn!("PaymentIntent failed: {}", payment_intent.id);
//                 // Marcar pending booking como fallido
//             }
//         }
//         _ => {
//             tracing::debug!("Unhandled event type: {:?}", event.type_);
//         }
//     }
//     (StatusCode::OK, Json(json!({"received": true})))
// }
