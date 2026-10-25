use {
    crate::{
        controllers::{
            cal::{fetch_and_detect_changes, fetch_booking},
            payments::{fulfill_payment, get_booking_payment_ids, transfer_booking_payments},
            users::{get_user_by_email_db, update_first_free_class},
        },
        models::{
            cal::BookingStatus,
            response::ResponseAPI,
            state::AppState,
            user::UserDB,
            webhook::{Attendee, BookingChange, CalWebhookEvent, RefundResponse, WebhookTrigger},
        },
        services::payments::{
            REFUND_MIN_NOTICE_HOURS, RefundDecision, is_safe_id, refund_decision,
            verify_cal_signature, verify_stripe_signature,
        },
    },
    axum::{
        Json,
        body::Bytes,
        extract::State,
        http::{HeaderMap, StatusCode},
        response::IntoResponse,
    },
    serde_json::Value,
    std::sync::Arc,
    stripe::{CreateRefund, PaymentIntentId, Refund, RequestStrategy},
    tokio::time::{Interval, MissedTickBehavior},
};

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

/// Procesa el reembolso de un booking cancelado
/// Retorna Ok(RefundResponse) si el reembolso fue exitoso
async fn process_refund(
    state: &AppState,
    booking_id: &String,
) -> Result<Vec<RefundResponse>, String> {
    tracing::info!("Procesando reembolso para booking: {}", booking_id);

    if !is_safe_id(booking_id) {
        let msg: String = format!(
            "Intento de seguridad bloqueado: booking_id inválido '{}'",
            booking_id
        );
        tracing::warn!("{}", msg); // Logueamos como advertencia
        return Err("ID de reserva inválido".to_string());
    }

    let payment_ids: Vec<String> = get_booking_payment_ids(state, booking_id).await?;

    if payment_ids.is_empty() {
        tracing::info!(
            "Booking {} sin pagos asociados, nada que reembolsar",
            booking_id
        );
        return Ok(Vec::new());
    }

    let booking = fetch_booking(state, booking_id).await?;

    match refund_decision(&booking, chrono::Utc::now()) {
        RefundDecision::Refund => {}
        RefundDecision::Transfer(new_uid) => {
            transfer_booking_payments(state, booking_id, &new_uid).await?;
            return Ok(Vec::new());
        }
        RefundDecision::LateCancellation => {
            tracing::warn!(
                "Booking {} cancelado por el alumno con menos de {}h de antelación: sin reembolso automático, revisar manualmente los pagos {:?}",
                booking_id,
                REFUND_MIN_NOTICE_HOURS,
                payment_ids
            );
            return Ok(Vec::new());
        }
        RefundDecision::NotCancelled => {
            tracing::warn!(
                "Booking {} no está cancelado en Cal.com ({:?}), no se reembolsa",
                booking_id,
                booking.status
            );
            return Ok(Vec::new());
        }
    }

    if !state.payment_options.auto_refunds_enabled {
        tracing::warn!(
            "Reembolso automático desactivado (AUTO_REFUNDS_ENABLED). Booking {} cancelado con pagos {:?}: revisar y reembolsar manualmente en Stripe",
            booking_id,
            payment_ids
        );
        return Ok(Vec::new());
    }

    let mut refunds: Vec<RefundResponse> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for stripe_id in payment_ids {
        let pi_id: PaymentIntentId = match stripe_id.parse() {
            Ok(id) => id,
            Err(e) => {
                errors.push(format!("Invalid PaymentIntent ID {}: {}", stripe_id, e));
                continue;
            }
        };

        let idempotent_client = state
            .stripe_client
            .clone()
            .with_strategy(RequestStrategy::Idempotent(format!("refund-{}", stripe_id)));

        match Refund::create(
            &idempotent_client,
            CreateRefund {
                payment_intent: Some(pi_id),
                ..Default::default()
            },
        )
        .await
        {
            Ok(refund) => {
                tracing::info!("Reembolso creado en Stripe: {:?}", refund);
                refunds.push(RefundResponse {
                    id: refund.id.to_string(),
                    amount: refund.amount,
                    currency: refund.currency.to_string(),
                    status: refund.status.clone(),
                    created: refund.created,
                });
            }
            Err(e) => errors.push(format!("Error creando reembolso de {}: {:?}", stripe_id, e)),
        }
    }

    if !errors.is_empty() {
        let msg: String = errors.join("; ");
        tracing::error!("{}", msg);
        return Err(msg);
    }

    Ok(refunds)
}

async fn process_reschedule(state: &AppState, new_uid: &str) -> Result<(), String> {
    let booking = fetch_booking(state, new_uid).await?;

    let Some(old_uid) = booking
        .rescheduled_from_uid
        .as_deref()
        .filter(|uid| !uid.is_empty())
    else {
        tracing::info!(
            "Booking {} sin reserva original, nada que trasladar",
            new_uid
        );
        return Ok(());
    };

    transfer_booking_payments(state, old_uid, new_uid).await?;
    Ok(())
}

/// Procesa un booking creado de tipo "free-class"
/// Para permitir solo 1 por persona
async fn process_created_free(
    state: &AppState,
    booking_id: &String,
    attendees: &[Attendee],
) -> Result<String, String> {
    let user_email: String = match attendees.first() {
        Some(attendee) => attendee.email.clone(),
        None => {
            let msg: String = format!("No attendees found for booking: {}", booking_id);
            tracing::error!("{}", msg);
            return Err(msg);
        }
    };

    tracing::info!(
        "Procesando booking gratuito creado: {} para usuario: {}",
        booking_id,
        user_email
    );

    // Obtener usuario de Firebase
    let mut user: UserDB = get_user_by_email_db(state, user_email.clone().as_str())
        .await
        .ok_or_else(|| {
            let msg = format!("Usuario no encontrado: {}", user_email.clone());
            tracing::error!("{}", msg);
            msg
        })?;

    // Verificar si el usuario ya tiene un booking gratuito confirmado
    user.first_free_class = true;

    // Actualizar solo el campo first_free_class
    update_first_free_class(state, user_email.as_str()).await?;

    tracing::info!("Clase gratuita marcada para: {}", user_email);

    Ok("First class to user reserved".to_string())
}

/// Obtener webhooks de cal
pub async fn handle_cal_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(secret) = state.payment_options.cal_webhook_secret.as_deref() else {
        tracing::error!("CAL_WEBHOOK_SECRET is not configured, rejecting Cal.com webhook");
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ResponseAPI::<()>::error(
                "Webhook not configured".to_string(),
            )),
        )
            .into_response();
    };

    let signature: Option<&str> = headers
        .get("x-cal-signature-256")
        .and_then(|value| value.to_str().ok());
    if !signature.is_some_and(|signature| verify_cal_signature(&body, signature, secret)) {
        tracing::warn!("Cal.com webhook with invalid signature rejected");
        return (
            StatusCode::UNAUTHORIZED,
            Json(ResponseAPI::<()>::error("Invalid signature".to_string())),
        )
            .into_response();
    }

    let event: CalWebhookEvent = match serde_json::from_slice(&body) {
        Ok(event) => event,
        Err(e) => {
            tracing::error!("Invalid Cal.com webhook payload: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseAPI::<()>::error("Invalid payload".to_string())),
            )
                .into_response();
        }
    };

    tracing::info!("📨 Webhook received: {}", event.trigger_event);
    tracing::debug!("Full payload: {:?}", event);

    // Si el evento es cancelación, procesamos refund
    match event.trigger_event {
        WebhookTrigger::BookingCancelled => {
            let uid: &String = match event.payload.uid.as_ref() {
                Some(id) => id,
                None => {
                    tracing::error!("Missing uid in cancelled booking payload");
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ResponseAPI::<()>::error("Missing booking uid".to_string())),
                    )
                        .into_response();
                }
            };

            tracing::info!(
                "Booking cancelled: {} - Reason: {:?}",
                uid,
                event.payload.cancellation_reason
            );

            match process_refund(&state, uid).await {
                Ok(refund_responses) => {
                    return (
                        StatusCode::OK,
                        Json(ResponseAPI::<Vec<RefundResponse>>::success(
                            "Cancellation processed".to_string(),
                            refund_responses,
                        )),
                    )
                        .into_response();
                }
                Err(err_msg) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<()>::error(err_msg)),
                    )
                        .into_response();
                }
            }
        }
        WebhookTrigger::BookingCreated => {
            let uid: &String = match event.payload.uid.as_ref() {
                Some(id) => id,
                None => {
                    tracing::error!("Missing uid in created booking payload");
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ResponseAPI::<()>::error("Missing booking uid".to_string())),
                    )
                        .into_response();
                }
            };

            tracing::info!("Booking created: {}", uid);
            if event.payload.event_type_slug.as_deref() == Some("free-class") {
                tracing::info!("New booking created: {}", uid);
                // Procesamos el booking creado
                match process_created_free(&state, uid, &event.payload.attendees).await {
                    Ok(success_msg) => {
                        tracing::info!(
                            "Booking gratuito creado procesado exitosamente - booking: {}",
                            uid
                        );
                        return (
                            StatusCode::OK,
                            Json(ResponseAPI::<String>::success(
                                "Free class booking processed successfully".to_string(),
                                success_msg,
                            )),
                        )
                            .into_response();
                    }
                    Err(err_msg) => {
                        tracing::error!(
                            "Error procesando booking gratuito creado - booking: {}, Error: {}",
                            uid,
                            err_msg
                        );
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ResponseAPI::<()>::error(err_msg)),
                        )
                            .into_response();
                    }
                }
            }
        }
        WebhookTrigger::BookingRescheduled => {
            let Some(uid) = event.payload.uid.as_ref() else {
                tracing::error!("Missing uid in rescheduled booking payload");
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error("Missing booking uid".to_string())),
                )
                    .into_response();
            };

            match process_reschedule(&state, uid).await {
                Ok(()) => {
                    return (StatusCode::OK, Json(ResponseAPI::<()>::success_no_data()))
                        .into_response();
                }
                Err(err_msg) => {
                    tracing::error!("Error procesando reprogramación de {}: {}", uid, err_msg);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<()>::error(err_msg)),
                    )
                        .into_response();
                }
            }
        }
        _ => {
            tracing::info!(
                "Webhook event received but no action taken: {}",
                event.trigger_event
            );
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

pub async fn handle_stripe_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(secret) = state.payment_options.stripe_webhook_secret.as_deref() else {
        tracing::error!("STRIPE_WEBHOOK_SECRET is not configured, rejecting Stripe webhook");
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ResponseAPI::<()>::error(
                "Webhook not configured".to_string(),
            )),
        )
            .into_response();
    };

    let signature: Option<&str> = headers
        .get("stripe-signature")
        .and_then(|value| value.to_str().ok());
    let now: i64 = chrono::Utc::now().timestamp();
    if !signature.is_some_and(|signature| verify_stripe_signature(&body, signature, secret, now)) {
        tracing::warn!("Stripe webhook with invalid signature rejected");
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error("Invalid signature".to_string())),
        )
            .into_response();
    }

    let event: Value = match serde_json::from_slice(&body) {
        Ok(event) => event,
        Err(e) => {
            tracing::error!("Invalid Stripe webhook payload: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseAPI::<()>::error("Invalid payload".to_string())),
            )
                .into_response();
        }
    };

    let event_type: &str = event["type"].as_str().unwrap_or_default();
    tracing::info!("📨 Stripe webhook received: {}", event_type);

    if event_type != "payment_intent.succeeded" {
        return (StatusCode::OK, Json(ResponseAPI::<()>::success_no_data())).into_response();
    }

    let Some(payment_intent_id) = event["data"]["object"]["id"].as_str() else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error(
                "Missing PaymentIntent id".to_string(),
            )),
        )
            .into_response();
    };

    match fulfill_payment(&state, payment_intent_id).await {
        Ok(()) => (StatusCode::OK, Json(ResponseAPI::<()>::success_no_data())).into_response(),
        Err(e) => {
            tracing::error!("Error fulfilling payment {}: {}", payment_intent_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error fulfilling payment".to_string(),
                )),
            )
                .into_response()
        }
    }
}

/// Tarea de polling para detectar cambios en bookings de Cal.com
pub async fn polling_task(state: Arc<AppState>) {
    let poll_interval_secs: u64 = 600; // 10 minutos
    let mut interval: Interval =
        tokio::time::interval(std::time::Duration::from_secs(poll_interval_secs));

    // MissedTickBehavior::Skip evita acumulación si el procesamiento tarda más que el intervalo
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    tracing::info!(
        "Cal.com polling task initiated (cada {}s)",
        poll_interval_secs
    );

    loop {
        // Esperar hasta el próximo tick (primera iteración es inmediata)
        interval.tick().await;

        match fetch_and_detect_changes(&state).await {
            Ok(changes) if !changes.is_empty() => {
                tracing::info!(
                    "Detected {} changes in Cal.com at {}",
                    changes.len(),
                    chrono::Utc::now()
                );
                for change in &changes {
                    tracing::info!(
                        "  Booking {}: {:?} -> {:?} (detected at: {})",
                        change.uid,
                        change.old_status,
                        change.new_status,
                        change.detected_at
                    );
                }

                // === BLOQUE DE ACTUALIZACIÓN DEL HISTORIAL ===
                // RwLock permite múltiples lectores simultáneos PERO solo un escritor a la vez.
                // Esto evita race conditions cuando varias tareas intentan leer/escribir.
                {
                    // .write() obtiene acceso EXCLUSIVO al Vec (bloquea a otros hasta que termine)
                    // Este "lock" se libera automáticamente al salir del bloque {}
                    let mut recent = state.cal_options.recent_changes.write().await;

                    // .iter().cloned(): itera sobre referencias y las clona
                    // (necesario porque usamos `changes` más abajo)
                    recent.extend(changes.iter().cloned());

                    // Limitar tamaño del historial
                    if recent.len() > 1000 {
                        tracing::debug!("Clearing cache (keeping last 500)");

                        // .drain(0..500): REMUEVE los primeros 500 elementos del Vec
                        // Es más eficiente que crear un Vec nuevo con .skip()
                        recent.drain(0..500);
                    }
                }

                // === PROCESAMIENTO DE CAMBIOS ===
                // Procesar cada cambio detectado
                for change in changes {
                    // Si handle_booking_change falla, logueamos el error
                    // pero continuamos procesando los demás cambios
                    if let Err(e) = handle_booking_change(&state, change).await {
                        tracing::error!("Error processing change: {}", e);
                    }
                }
            }
            Ok(_) => {
                tracing::info!("No changes detected in Cal.com");
            }
            Err(e) => {
                tracing::error!("Error in Cal.com polling: {}", e);
            }
        }
    }
}

/// Maneja un cambio de booking detectado en polling
async fn handle_booking_change(state: &AppState, change: BookingChange) -> Result<(), String> {
    match change.new_status {
        BookingStatus::Cancelled | BookingStatus::Rejected => {
            tracing::info!("Polling detectó cancelación - booking: {}", change.uid);

            // Procesar el reembolso usando la misma lógica que el webhook
            match process_refund(state, &change.uid).await {
                Ok(refund_responses) => {
                    for refund_response in refund_responses {
                        tracing::info!(
                            "Reembolso procesado exitosamente desde polling - Refund ID: {}, Amount: {}, Status: {:?}",
                            refund_response.id,
                            refund_response.amount,
                            refund_response.status
                        );
                    }
                    Ok(())
                }
                Err(err_msg) => {
                    tracing::error!("Error procesando reembolso desde polling: {}", err_msg);
                    Err(err_msg)
                }
            }
        }
        _ => {
            tracing::debug!(
                "Cambio de estado detectado HTTP Polling: {:?} -> {:?}",
                change.uid,
                change.new_status
            );
            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "../test/controllers/webhook.rs"]
mod extended_tests;
