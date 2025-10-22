use {
    crate::{
        controllers::{
            cal::fetch_and_detect_changes,
            users::{get_user_by_email_db, update_first_free_class},
        },
        models::{
            cal::BookingStatus,
            response::ResponseAPI,
            state::AppState,
            stripe::RelationalCalStripe,
            user::UserDB,
            webhook::{Attendee, BookingChange, CalWebhookEvent, RefundResponse, WebhookTrigger},
        },
    },
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    std::sync::Arc,
    stripe::{CreateRefund, PaymentIntentId, Refund},
    tokio::time::{Interval, MissedTickBehavior},
};

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

/// Procesa el reembolso de un booking cancelado
/// Retorna Ok(RefundResponse) si el reembolso fue exitoso
async fn process_refund(state: &AppState, booking_id: &str) -> Result<RefundResponse, String> {
    tracing::info!("Procesando reembolso para booking: {}", booking_id);

    let url_firebase_db_relationship: String = format!(
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
            tracing::info!("Reembolso creado en Stripe: {:?}", refund);

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

    tracing::info!("Clase gratuita marcada para: {}", user_email);

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
                "Booking cancelled: {} - Reason: {:?}",
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
            tracing::info!("Booking created: {}", event.payload.uid);
            if event.payload.event_type_slug.as_deref() == Some("free-class") {
                tracing::info!("New booking created: {}", event.payload.uid);
                // Procesamos el booking creado
                match process_created_free(&state, &event.payload.uid, &event.payload.attendees)
                    .await
                {
                    Ok(success_msg) => {
                        tracing::info!(
                            "Booking gratuito creado procesado exitosamente - booking: {}",
                            event.payload.uid
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
        BookingStatus::Cancelled => {
            tracing::info!("Polling detectó cancelación - booking: {}", change.uid);

            // Procesar el reembolso usando la misma lógica que el webhook
            match process_refund(state, &change.uid).await {
                Ok(refund_response) => {
                    tracing::info!(
                        "Reembolso procesado exitosamente desde polling - Refund ID: {}, Amount: {}, Status: {:?}",
                        refund_response.id,
                        refund_response.amount,
                        refund_response.status
                    );
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
