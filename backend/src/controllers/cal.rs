use {
    crate::models::{
        cal::{
            AddGuestsPayload, BookingsQueryParams, CalApiResponse, CalBookingPayload,
            FetchCalErrors, Schedule, UserCal,
        },
        response::ResponseAPI,
        state::AppState,
        webhook::{BookingChange, CalBookingsResponse},
    },
    axum::{
        Json, debug_handler,
        extract::{Path, Query, State},
        http::StatusCode,
        response::{IntoResponse, Response as AxumResponse},
    },
    chrono::Utc,
    reqwest::{Response, Url},
    serde_json::{Value, json},
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLockWriteGuard,
    tracing::{debug, error, info, instrument},
};

/// Helper: Maneja errores de red al comunicarse con Cal.com
fn handle_network_error(error: &reqwest::Error) -> AxumResponse {
    error!(error = %error, "Failed to communicate with Cal.com");
    (
        StatusCode::BAD_GATEWAY,
        Json(ResponseAPI::<()>::error(
            "Failed to communicate with Cal.com".to_string(),
        )),
    )
        .into_response()
}

/// Helper: Maneja respuestas de error de Cal.com API
async fn handle_cal_error_response(response: Response) -> AxumResponse {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    error!(status = %status, body = %body, "Cal.com returned error");
    (
        StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(ResponseAPI::<()>::error(format!("Cal.com error: {}", body))),
    )
        .into_response()
}

/// Confirmar un booking
#[debug_handler]
#[instrument(skip(state), fields(booking_id = %id))]
pub async fn confirm_booking(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let url: String = format!("{}/bookings/{}/confirm", state.cal_options.base_url, id);

    // Obtenemos la respuesta de la api externa de cal
    let response: Response = match state
        .cal_options
        .client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("cal-api-version", "2024-06-11")
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await
    {
        Ok(r) => r,
        Err(ref e) => return handle_network_error(e),
    };

    // Manejamos los posibles errores de la respuesta
    if !response.status().is_success() {
        return handle_cal_error_response(response).await;
    }

    info!("Booking confirmed successfully");
    (
        StatusCode::NO_CONTENT,
        Json(ResponseAPI::<()>::success_no_data()),
    )
        .into_response()
}

/// Obtener últimos bookings para HTTP Polling
#[instrument(skip(client, api_key))]
async fn fetch_cal_bookings_internal(
    client: &reqwest::Client,
    api_key: &str,
    api_version: &str,
) -> Result<Vec<CalBookingPayload>, FetchCalErrors> {
    let response: Response = client
        .get("https://api.cal.com/v2/bookings")
        .header("Authorization", api_key)
        .header("cal-api-version", api_version)
        .query(&[("take", "10"), ("sortUpdatedAt", "desc")])
        .send()
        .await
        .map_err(FetchCalErrors::Network)?;

    let bookings_response = response
        .json::<CalBookingsResponse>()
        .await
        .map_err(FetchCalErrors::ParseError)?;

    Ok(bookings_response.data.bookings)
}

/// Comoparear cambios para HTTP Polling
#[instrument(skip(state))]
pub async fn fetch_and_detect_changes(state: &AppState) -> Result<Vec<BookingChange>, String> {
    // 1. Fetch bookings desde Cal.com API
    let current_bookings: Vec<CalBookingPayload> = fetch_cal_bookings_internal(
        &state.cal_options.client,
        &state.cal_options.api_key,
        "2024-06-11",
    )
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to fetch bookings");
        format!("Failed to fetch bookings: {}", e)
    })?;

    // 2. Leer caché actual
    let mut cache: RwLockWriteGuard<HashMap<String, CalBookingPayload>> =
        state.cal_options.booking_cache.write().await;
    let mut changes: Vec<BookingChange> = Vec::new();

    // 3. Detectar cambios
    for booking in current_bookings {
        // Solo procesar bookings que tengan uid
        if let Some(uid) = &booking.uid {
            // Normalize UID to String so it matches the HashMap<String, _> key type
            let uid_str: String = uid.to_string();

            if let Some(cached) = cache.get(&uid_str) {
                // Booking existe - verificar si cambió el status
                if cached.status != booking.status {
                    info!(
                        booking_uid = %uid,
                        old_status = ?cached.status,
                        new_status = ?booking.status,
                        "Booking status changed"
                    );

                    changes.push(BookingChange {
                        uid: uid.clone(),
                        old_status: cached.status.clone(),
                        new_status: booking.status.clone(),
                        detected_at: Utc::now(),
                    });
                }
            }

            // Actualizar caché
            cache.insert(uid_str.clone(), booking);
        }
    }

    if !changes.is_empty() {
        info!(changes_count = changes.len(), "Detected booking changes");
    }

    Ok(changes)
}

/// Obtener un booking por ID
#[debug_handler]
#[instrument(skip(state), fields(booking_id = %id))]
pub async fn get_booking(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let url: String = format!("{}/bookings/{}", state.cal_options.base_url, id);

    // Realizar la petición GET a Cal.com
    let response: Response = match state
        .cal_options
        .client
        .get(&url)
        .header("Authorization", &state.cal_options.api_key)
        .header("cal-api-version", "2024-06-11")
        .send()
        .await
    {
        Ok(response) => response,
        Err(ref e) => return handle_network_error(e),
    };

    // Manejar errores de la respuesta
    if !response.status().is_success() {
        return handle_cal_error_response(response).await;
    }

    // Obtener el texto para el logging
    let response_text: String = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            error!("Failed to read response body for booking {}: {}", id, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<CalBookingPayload>::error(
                    "Failed to read Cal.com response".to_string(),
                )),
            )
                .into_response();
        }
    };

    debug!(
        "Raw Cal.com GET response for booking {}: {}",
        id, response_text
    );

    match serde_json::from_str::<CalApiResponse<CalBookingPayload>>(&response_text) {
        Ok(b) => (
            StatusCode::OK,
            Json(ResponseAPI::<CalBookingPayload>::success(
                "Booking retrieved successfully".to_string(),
                b.data,
            )),
        )
            .into_response(),
        Err(e) => {
            error!(
                "Failed to parse Cal.com response for booking {}: {} - Body: {}",
                id, e, response_text
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<CalBookingPayload>::error(
                    "Invalid booking data format".to_string(),
                )),
            )
                .into_response();
        }
    }
}

/// Agregar invitados/asistentes a un booking existente
#[debug_handler]
#[instrument(skip(state, payload), fields(booking_id = %id))]
pub async fn add_guests_to_booking(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<AddGuestsPayload>,
) -> impl IntoResponse {
    if payload.guests.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error(
                "At least one guest is required".to_string(),
            )),
        )
            .into_response();
    }

    let url: String = format!("{}/bookings/{}/guests", state.cal_options.base_url, id);

    info!(
        booking_id = %id,
        url = %url,
        guests_count = payload.guests.len(),
        body = %serde_json::to_string_pretty(&payload).unwrap_or_default(),
        "Adding guests to booking"
    );

    // Realizar la petición POST a Cal.com enviando el objeto con propiedad "guests"
    let response: Response = match state
        .cal_options
        .client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("cal-api-version", "2024-08-13")
        .header("Authorization", &state.cal_options.api_key)
        .json(&payload)
        .send()
        .await
    {
        Ok(r) => r,
        Err(ref e) => return handle_network_error(e),
    };

    // Verificar el status de la respuesta
    if !response.status().is_success() {
        return handle_cal_error_response(response).await;
    }

    // Parsear la respuesta exitosa
    match response.json::<CalApiResponse<CalBookingPayload>>().await {
        Ok(cal_response) => {
            info!(
                booking_id = %id,
                guests_added = payload.guests.len(),
                "Guests added successfully to booking"
            );
            (
                StatusCode::OK,
                Json(ResponseAPI::<CalBookingPayload>::success(
                    format!("{} guest(s) added successfully", payload.guests.len()),
                    cal_response.data,
                )),
            )
                .into_response()
        }
        Err(e) => {
            error!(error = %e, "Failed to parse Cal.com add guests response");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Invalid response format from Cal.com".to_string(),
                )),
            )
                .into_response()
        }
    }
}

/// Agregar un nuevo booking
#[debug_handler]
pub async fn add_booking(
    State(state): State<Arc<AppState>>,
    Json(flexible_payload): Json<Value>,
) -> impl IntoResponse {
    // Intentar parsear el payload flexible
    let mut payload: CalBookingPayload =
        match serde_json::from_value::<CalBookingPayload>(flexible_payload.clone()) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to parse booking payload: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error(format!(
                        "Invalid booking payload: {}",
                        e
                    ))),
                )
                    .into_response();
            }
        };

    // Si viene username como string plano, extraerlo y crear el objeto user
    if let Some(username_str) = flexible_payload.get("username").and_then(|v| v.as_str()) {
        if payload.user.is_none() {
            payload.user = Some(UserCal {
                id: 0,
                username: username_str.to_string(),
                email: String::new(),
                time_zone: None,
            });
        }
    }

    // Validaciones
    if payload.attendees.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error(
                "At least one attendee is required".to_string(),
            )),
        )
            .into_response();
    }

    if payload.start_time.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error(
                "Start time is required".to_string(),
            )),
        )
            .into_response();
    }

    let attendee = &payload.attendees[0];
    let start_time = payload.start_time.as_ref().unwrap();

    // Validar que se cumple al menos uno de los requisitos de Cal.com:
    // 1. eventTypeId solo
    // 2. eventTypeSlug + username
    // 3. eventTypeSlug + teamSlug
    let has_event_type_id = payload.event_type_id.is_some();
    let has_event_type_slug = payload.event_type_slug.is_some();
    let has_username = payload
        .user
        .as_ref()
        .map(|u| !u.username.is_empty())
        .unwrap_or(false);
    let has_team_slug = payload
        .team_slug
        .as_ref()
        .map(|t| !t.is_empty())
        .unwrap_or(false);

    let valid_combination = has_event_type_id
        || (has_event_type_slug && has_username)
        || (has_event_type_slug && has_team_slug);

    if !valid_combination {
        return (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error(
                "Either eventTypeId or (eventTypeSlug + username) or (eventTypeSlug + teamSlug) must be provided".to_string(),
            )),
        )
            .into_response();
    }

    // Construir el body dinámicamente según los campos disponibles
    let mut body = json!({
        "start": start_time,
        "attendee": {
            "name": attendee.name,
            "email": attendee.email,
            "timeZone": attendee.time_zone.as_str(),
            "language": attendee.language.as_ref().map(|l| l.locale.as_str()).unwrap_or("es"),
        },
        "metadata": payload.metadata.clone().unwrap_or(json!({})),
    });

    // Añadir location solo si está presente en el payload
    if let Some(location) = &payload.location {
        body["location"] = serde_json::from_str::<Value>(location)
            .unwrap_or_else(|_| Value::String(location.clone()));
    }

    // Añadir title si está presente
    if let Some(title) = &payload.title {
        body["title"] = json!(title);
    }

    // Nota: Cal.com API v2 no acepta el campo 'notes' directamente
    // Si necesitas agregar notas, deben ir en metadata o bookingFieldsResponses

    // Añadir campos opcionales según estén disponibles
    if let Some(event_type_id) = payload.event_type_id {
        body["eventTypeId"] = json!(event_type_id);
    }
    if let Some(event_type_slug) = &payload.event_type_slug {
        body["eventTypeSlug"] = json!(event_type_slug);
    }
    if let Some(user) = &payload.user {
        body["username"] = json!(user.username);
    }
    if let Some(team_slug) = &payload.team_slug {
        body["teamSlug"] = json!(team_slug);
    }
    if let Some(organization_slug) = &payload.organization_slug {
        body["organizationSlug"] = json!(organization_slug);
    }
    // Añadir invitados si están presentes
    if let Some(guests) = &payload.guests {
        if !guests.is_empty() {
            body["guests"] = json!(guests);
            info!("Adding {} guest(s) to booking: {:?}", guests.len(), guests);
        }
    }

    info!(
        "Creating Cal.com booking with body: {}",
        serde_json::to_string_pretty(&body).unwrap_or_default()
    );

    let url: String = format!("{}/bookings", state.cal_options.base_url);

    let response = state
        .cal_options
        .client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("cal-api-version", "2024-08-13")
        .header("Authorization", &state.cal_options.api_key)
        .json(&body)
        .send()
        .await;

    debug!("Cal.com response: {:?}", response);

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text: String = resp.text().await.unwrap_or_default();

            if status.is_success() {
                debug!("Raw Cal.com success response body: {}", text);
                match serde_json::from_str::<CalApiResponse<CalBookingPayload>>(&text) {
                    Ok(cal_response) => (
                        StatusCode::CREATED,
                        Json(ResponseAPI::success(
                            "Booking created successfully".to_string(),
                            cal_response.data,
                        )),
                    )
                        .into_response(),
                    Err(e) => {
                        tracing::error!("Parse error: {} - Body: {}", e, text);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ResponseAPI::<()>::error(format!(
                                "Error parsing response: {}",
                                e
                            ))),
                        )
                            .into_response()
                    }
                }
            } else {
                tracing::error!("Cal.com error ({}): {}", status, text);
                (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_REQUEST),
                    Json(ResponseAPI::<()>::error(format!("Cal.com error: {}", text))),
                )
                    .into_response()
            }
        }
        Err(e) => {
            tracing::error!("Request failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!("Request failed: {}", e))),
            )
                .into_response()
        }
    }
}

/// Obtener todos los calendarios del usuario autenticado
#[debug_handler]
#[instrument(skip(state))]
pub async fn get_schedules(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let response = state
        .cal_options
        .client
        .get(format!("{}/schedules", state.cal_options.base_url))
        .header("cal-api-version", "2024-06-11")
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                return handle_cal_error_response(resp).await;
            }

            match resp.json::<CalApiResponse<Vec<Schedule>>>().await {
                Ok(cal_response) => (
                    StatusCode::OK,
                    Json(ResponseAPI::<Vec<Schedule>>::success(
                        "Schedules retrieved successfully".to_string(),
                        cal_response.data,
                    )),
                )
                    .into_response(),
                Err(e) => {
                    error!("Parse error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<()>::error(format!(
                            "Error parsing response: {}",
                            e
                        ))),
                    )
                        .into_response()
                }
            }
        }
        Err(ref e) => handle_network_error(e),
    }
}

/// Obtener un calendario específico
#[debug_handler]
#[instrument(skip(state))]
pub async fn get_schedule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let url: String = format!("{}/schedules/{}", state.cal_options.base_url, id);

    tracing::info!(
        schedule_id = %id,
        url = %url,
        "Starting get_schedule request"
    );

    let response: Result<Response, reqwest::Error> = state
        .cal_options
        .client
        .get(&url)
        .header("cal-api-version", "2024-06-11")
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                return handle_cal_error_response(resp).await;
            }

            match resp.json::<CalApiResponse<Schedule>>().await {
                Ok(cal_response) => (
                    StatusCode::OK,
                    Json(ResponseAPI::success(
                        "Schedule retrieved successfully".to_string(),
                        cal_response.data,
                    )),
                )
                    .into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(format!(
                        "Error parsing response: {}",
                        e
                    ))),
                )
                    .into_response(),
            }
        }
        Err(ref e) => handle_network_error(e),
    }
}

/// Obtener todos los bookings
#[debug_handler]
#[instrument(skip(state))]
pub async fn get_all_bookings(
    State(state): State<Arc<AppState>>,
    Query(params): Query<BookingsQueryParams>,
) -> impl IntoResponse {
    let mut url: Url =
        match reqwest::Url::parse(&format!("{}/bookings", state.cal_options.base_url)) {
            Ok(u) => u,
            Err(e) => {
                tracing::error!("Error parsing URL for bookings: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error(format!("Invalid base URL: {}", e))),
                )
                    .into_response();
            }
        };

    {
        let mut query = url.query_pairs_mut();

        if let Some(id) = params.event_type_id {
            query.append_pair("eventTypeId", &id);
        }
        if let Some(ids) = params.event_type_ids {
            query.append_pair("eventTypeIds", &ids);
        }
        if let Some(email) = params.attendee_email {
            query.append_pair("attendeeEmail", &email);
        }
        if let Some(name) = params.attendee_name {
            query.append_pair("attendeeName", &name);
        }
        if let Some(team_id) = params.team_id {
            query.append_pair("teamId", &team_id);
        }
        if let Some(after) = params.after_start {
            query.append_pair("afterStart", &after);
        }
        if let Some(before) = params.before_end {
            query.append_pair("beforeEnd", &before);
        }
        if let Some(status) = params.status {
            query.append_pair("status", &status);
        }
        if let Some(sort) = params.sort_start {
            query.append_pair("sortStart", &sort);
        }
    }

    let response = state
        .cal_options
        .client
        .get(url)
        .header("cal-api-version", "2024-06-11")
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                return handle_cal_error_response(resp).await;
            }

            match resp.json::<CalBookingsResponse>().await {
                Ok(cal_response) => (
                    StatusCode::OK,
                    Json(ResponseAPI::<Vec<CalBookingPayload>>::success(
                        "Bookings retrieved successfully".to_string(),
                        cal_response.data.bookings,
                    )),
                )
                    .into_response(),
                Err(e) => {
                    error!("Parse error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseAPI::<()>::error(format!(
                            "Error parsing response: {}",
                            e
                        ))),
                    )
                        .into_response()
                }
            }
        }
        Err(ref e) => handle_network_error(e),
    }
}

#[cfg(test)]
#[path = "../test/controllers/cal.rs"]
mod tests;
