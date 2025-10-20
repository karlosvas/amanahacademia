use {
    crate::{
        controllers::{
            cal::fetch_and_detect_changes,
            users::{get_user_by_email_db, update_first_free_class},
        },
        models::{
            cal::BookingStatus,
            response::ResponseAPI,
            stripe::RelationalCalStripe,
            user::UserDB,
            webhook::{Attendee, BookingChange, CalWebhookEvent, RefundResponse, WebhookTrigger},
        },
        state::AppState,
    },
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
    stripe::{CreateRefund, PaymentIntentId, Refund},
    tokio::time::Interval,
};

// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

/// Procesa el reembolso de un booking cancelado
/// Retorna Ok(RefundResponse) si el reembolso fue exitoso
async fn process_refund(state: &AppState, booking_id: &str) -> Result<RefundResponse, String> {
    tracing::info!("Procesando reembolso para booking: {}", booking_id);

    let url_firebase_db_relationship = format!(
        "{}/relation_cal_stripe/{}.json",
        state.firebase_options.firebase_database_url, booking_id
    );

    // Obtener relación booking <-> Stripe desde Firebase
    let stripe_id: String = match state
        .firebase_options
        .firebase_client
        .get(&url_firebase_db_relationship)
        .send()
        .await
    {
        Ok(res) => match res.json::<Option<RelationalCalStripe>>().await {
            Ok(Some(val)) => val.stripe_id,
            Ok(None) => {
                let msg = format!(
                    "No se encontró relación de Firebase para booking_id: {}",
                    booking_id
                );
                tracing::error!("{}", msg);
                return Err(msg);
            }
            Err(e) => {
                let msg = format!("Error parseando respuesta de Firebase: {}", e);
                tracing::error!("{}", msg);
                return Err(msg);
            }
        },
        Err(e) => {
            let msg = format!("Error obteniendo relación de Firebase: {}", e);
            tracing::error!("{}", msg);
            return Err(msg);
        }
    };

    // Convertir a PaymentIntentId
    let pi_id: PaymentIntentId = match stripe_id.parse() {
        Ok(id) => id,
        Err(e) => {
            let msg = format!("Invalid PaymentIntent ID: {}", e);
            tracing::error!("{}", msg);
            return Err(msg);
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
            tracing::info!("✅ Reembolso creado en Stripe: {:?}", refund);

            let response = RefundResponse {
                id: refund.id.to_string(),
                amount: refund.amount,
                currency: refund.currency.to_string(),
                status: refund.status.clone(),
                created: refund.created,
            };
            Ok(response)
        }
        Err(e) => {
            let msg = format!("Error creando reembolso: {:?}", e);
            tracing::error!("{}", msg);
            Err(msg)
        }
    }
}

/// Procesa un booking creado de tipo "free-class"
/// Para permitir solo 1 por persona
async fn process_created_free(
    state: &AppState,
    booking_id: &str,
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

    tracing::info!("✅ Clase gratuita marcada para: {}", user_email);

    Ok("First class to user reserved".to_string())
}

/// Obtener webhooks de cal
pub async fn handle_cal_webhook(
    State(state): State<Arc<AppState>>,
    Json(event): Json<CalWebhookEvent>,
) -> impl IntoResponse {
    tracing::info!("📨 Webhook received: {}", event.trigger_event);
    tracing::debug!("Full payload: {:?}", event);

    // Si el evento es cancelación, procesamos refund
    match event.trigger_event {
        WebhookTrigger::BookingCancelled => {
            tracing::info!(
                "📨 Booking cancelled: {} - Reason: {:?}",
                event.payload.uid,
                event.payload.cancellation_reason
            );

            match process_refund(&state, &event.payload.uid).await {
                Ok(refund_response) => {
                    return (
                        StatusCode::OK,
                        Json(ResponseAPI::<RefundResponse>::success(
                            "Refund created successfully".to_string(),
                            refund_response,
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
            if event.payload.event_type_slug == "free-class" {
                tracing::info!("📨 New booking created: {}", event.payload.uid);
                // Procesamos el booking creado
                match process_created_free(&state, &event.payload.uid, &event.payload.attendees)
                    .await
                {
                    Ok(_) => {
                        tracing::info!(
                            "Booking gratuito creado procesado exitosamente - booking: {}",
                            event.payload.uid
                        );
                    }
                    Err(err_msg) => {
                        tracing::error!(
                            "Error procesando booking gratuito creado - booking: {}, Error: {}",
                            event.payload.uid,
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
        _ => {
            tracing::info!(
                "📨 Webhook event received but no action taken: {}",
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

/// Tarea de polling para detectar cambios en bookings de Cal.com
pub async fn polling_task(state: Arc<AppState>) {
    let mut interval: Interval = tokio::time::interval(std::time::Duration::from_secs(60));

    // Primera tick es inmediata, la saltamos
    interval.tick().await;

    tracing::info!("📊 Cal.com polling task iniciada (cada {}s)", 60);

    loop {
        interval.tick().await;

        let changes_result: Result<Vec<BookingChange>, String> =
            fetch_and_detect_changes(&state).await;

        match changes_result {
            Ok(changes) => {
                if !changes.is_empty() {
                    tracing::info!("🔍 Detectados {} cambios en Cal.com", changes.len());

                    // Clonar los cambios antes de procesarlos
                    let changes_to_process = changes.clone();

                    // Almacenar cambios para consulta posterior
                    {
                        let mut recent = state.cal_options.recent_changes.write().await;
                        recent.extend(changes);

                        // Opcional: mantener solo últimos 1000 cambios
                        if recent.len() > 1000 {
                            tracing::debug!(
                                "🗑️  Limpiando caché de cambios recientes (manteniendo últimos 500)"
                            );
                            recent.drain(0..500);
                        }
                    } // El lock se libera aquí

                    // Procesar cada cambio
                    for change in changes_to_process {
                        handle_booking_change(&state, change).await;
                    }
                }
            }
            Err(e) => {
                tracing::error!("Error en polling de Cal.com: {}", e);
            }
        }
    }
}

/// Maneja un cambio de booking detectado en polling
async fn handle_booking_change(state: &AppState, change: BookingChange) {
    match change.new_status {
        BookingStatus::Cancelled => {
            tracing::info!("🔄 Polling detectó cancelación - booking: {}", change.uid);

            // Procesar el reembolso usando la misma lógica que el webhook
            match process_refund(state, &change.uid).await {
                Ok(refund_response) => {
                    tracing::info!(
                        "✅ Reembolso procesado exitosamente desde polling - Refund ID: {}, Amount: {}, Status: {:?}",
                        refund_response.id,
                        refund_response.amount,
                        refund_response.status
                    );
                }
                Err(err_msg) => {
                    tracing::error!("Error procesando reembolso desde polling: {}", err_msg);
                }
            }
        }
        _ => {
            tracing::debug!(
                "Cambio de estado detectado HTTP Polling: {:?} -> {:?}",
                change.uid,
                change.new_status
            );
        }
    }
}
