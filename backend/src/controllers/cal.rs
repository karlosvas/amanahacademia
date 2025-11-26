use {
    crate::models::{
        cal::CalApiResponse,
        response::ResponseAPI,
        state::AppState,
        webhook::{BookingChange, CalBookingPayload, CalBookingsResponse},
    },
    axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    chrono::Utc,
    reqwest::Response,
    serde::Deserialize,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLockWriteGuard,
};

/// Confirmar un booking
pub async fn confirm_booking(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let url_cal: String = format!("{}/bookings/{}/confirm", state.cal_options.base_url, id);

    match state
        .cal_options
        .client
        .post(&url_cal)
        .header("Content-Type", "application/json")
        .header("cal-api-version", &state.cal_options.api_version)
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                tracing::info!("Booking {} confirmado exitosamente en Cal.com", id);
                (StatusCode::NO_CONTENT).into_response()
            } else {
                let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                let text: String = response.text().await.unwrap_or_default();
                tracing::error!(
                    "Error confirmando booking {} en Cal.com: {} - {}",
                    id,
                    status,
                    text
                );
                (
                    status,
                    Json(ResponseAPI::<()>::error(
                        "Error confirmando booking".to_string(),
                    )),
                )
                    .into_response()
            }
        }
        Err(e) => {
            tracing::error!(
                "Error haciendo la petición a Cal.com para confirmar booking {}: {}",
                id,
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error haciendo la petición a Cal.com".to_string(),
                )),
            )
                .into_response()
        }
    }
}

/// Obtener últimos bookings para HTTP Polling
async fn fetch_cal_bookings_internal(
    client: &reqwest::Client,
    api_key: &str,
    api_version: &str,
) -> Result<Vec<CalBookingPayload>, String> {
    let response: Response = client
        .get("https://api.cal.com/v2/bookings")
        .header("Authorization", api_key)
        .header("cal-api-version", api_version)
        .query(&[("take", "100"), ("sortUpdatedAt", "desc")])
        .send()
        .await
        .map_err(|e| format!("Error fetching bookings: {}", e))?;

    // Cal.com API v2 devuelve un objeto con estructura { "data": [...] }
    let bookings_response: CalBookingsResponse = response
        .json::<CalBookingsResponse>()
        .await
        .map_err(|e| format!("Error parsing bookings JSON: {}", e))?;

    Ok(bookings_response.data)
}

// Comoparear cambios para HTTP Polling
pub async fn fetch_and_detect_changes(state: &AppState) -> Result<Vec<BookingChange>, String> {
    // 1. Fetch bookings desde Cal.com API
    let current_bookings: Vec<CalBookingPayload> = fetch_cal_bookings_internal(
        &state.cal_options.client,
        &state.cal_options.api_key,
        &state.cal_options.api_version,
    )
    .await?;

    // 2. Leer caché actual
    let mut cache: RwLockWriteGuard<HashMap<String, CalBookingPayload>> =
        state.cal_options.booking_cache.write().await;
    let mut changes: Vec<BookingChange> = Vec::new();

    // 3. Detectar cambios
    for booking in current_bookings {
        if let Some(cached) = cache.get(&booking.uid) {
            // Booking existe - verificar si cambió el status
            if cached.status != booking.status {
                changes.push(BookingChange {
                    uid: booking.uid.clone(),
                    old_status: cached.status.clone(),
                    new_status: booking.status.clone(),
                    detected_at: Utc::now(),
                });
            }
        }

        // Actualizar caché
        cache.insert(booking.uid.clone(), booking);
    }

    Ok(changes)
}

#[axum::debug_handler]
pub async fn get_booking(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let resp_result: Result<Response, String> = state
        .cal_options
        .client
        .get(format!("https://api.cal.com/v2/bookings/{}", id))
        .header("Authorization", &state.cal_options.api_key)
        .header("cal-api-version", &state.cal_options.api_version)
        .send()
        .await
        .map_err(|e| format!("Error fetching booking: {}", e));

    match resp_result {
        Ok(resp) => match resp.text().await {
            Ok(body) => match serde_json::from_str::<CalApiResponse<CalBookingPayload>>(&body) {
                Ok(cal_response) => (
                    StatusCode::OK,
                    Json(ResponseAPI::<CalBookingPayload>::success(
                        "Booking fetched successfully".to_string(),
                        cal_response.data,
                    )),
                )
                    .into_response(),
                Err(e) => {
                    eprintln!("Error deserializando: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<CalBookingPayload>::error(format!(
                            "Error parsing booking JSON: {}",
                            e
                        ))),
                    )
                        .into_response()
                }
            },
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<CalBookingPayload>::error(format!(
                    "Error reading response body: {}",
                    e
                ))),
            )
                .into_response(),
        },
        Err(err_msg) => (
            StatusCode::BAD_GATEWAY,
            Json(ResponseAPI::<CalBookingPayload>::error(format!(
                "Error fetching booking: {}",
                err_msg
            ))),
        )
            .into_response(),
    }
}
