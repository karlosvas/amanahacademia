use {
    crate::models::{
        cal::{BookingsQueryParams, CalApiResponse, CalBookingPayload, Schedule},
        response::ResponseAPI,
        state::AppState,
        webhook::{BookingChange, CalBookingsResponse},
    },
    axum::{
        Json,
        extract::{Path, Query, State},
        http::StatusCode,
        response::IntoResponse,
    },
    chrono::Utc,
    reqwest::{Response, Url},
    serde_json::{Value, json},
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLockWriteGuard,
    tracing::debug,
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
        .query(&[("take", "10"), ("sortUpdatedAt", "desc")])
        .send()
        .await
        .map_err(|e| format!("Error fetching bookings: {}", e))?;

    // Obtener el texto de la respuesta primero para debugging
    let response_text: String = response
        .text()
        .await
        .map_err(|e| format!("Error reading response body: {}", e))?;

    // Intentar parsear como CalBookingsResponse
    match serde_json::from_str::<CalBookingsResponse>(&response_text) {
        Ok(bookings_response) => Ok(bookings_response.data.bookings),
        Err(e) => {
            // Si falla, intentar parsear como un array directo (por si la API cambió)
            match serde_json::from_str::<Vec<CalBookingPayload>>(&response_text) {
                Ok(bookings) => {
                    debug!("Cal.com API returned array directly instead of wrapped response");
                    Ok(bookings)
                }
                Err(_) => Err(format!(
                    "Error parsing bookings JSON: {}. Response: {}",
                    e, response_text
                )),
            }
        }
    }
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
        // Solo procesar bookings que tengan uid
        if let Some(uid) = &booking.uid {
            // Normalize UID to String so it matches the HashMap<String, _> key type
            let uid_str: String = uid.to_string();

            if let Some(cached) = cache.get(&uid_str) {
                // Booking existe - verificar si cambió el status
                if cached.status != booking.status {
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

    Ok(changes)
}

/// Obtener un booking por ID
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
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<CalBookingPayload>::error(format!(
                        "Error parsing booking JSON: {}",
                        e
                    ))),
                )
                    .into_response(),
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

/// Agregar un nuevo booking
#[axum::debug_handler]
pub async fn add_booking(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CalBookingPayload>,
) -> impl IntoResponse {
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

    // Añadir campos opcionales según estén disponibles
    if let Some(event_type_id) = payload.event_type_id {
        body["eventTypeId"] = json!(event_type_id);
    }
    if let Some(event_type_slug) = &payload.event_type_slug {
        body["eventTypeSlug"] = json!(event_type_slug);
    }
    if let Some(username) = &payload.username {
        body["username"] = json!(username);
    }
    if let Some(team_slug) = &payload.team_slug {
        body["teamSlug"] = json!(team_slug);
    }
    if let Some(organization_slug) = &payload.organization_slug {
        body["organizationSlug"] = json!(organization_slug);
    }

    let response = state
        .cal_options
        .client
        .post("https://api.cal.com/v2/bookings")
        .header("Content-Type", "application/json")
        .header("cal-api-version", &state.cal_options.api_version)
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
pub async fn get_schedules(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let response = state
        .cal_options
        .client
        .get(format!("{}/schedules", state.cal_options.base_url))
        .header("cal-api-version", &state.cal_options.api_version)
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text: String = resp.text().await.unwrap_or_default();

            if status.is_success() {
                match serde_json::from_str::<CalApiResponse<Vec<Schedule>>>(&text) {
                    Ok(cal_response) => (
                        StatusCode::OK,
                        Json(ResponseAPI::<Vec<Schedule>>::success(
                            "Schedules retrieved successfully".to_string(),
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

/// Obtener un calendario específico
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
        .header("cal-api-version", &state.cal_options.api_version)
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text: String = resp.text().await.unwrap_or_default();

            if status.is_success() {
                match serde_json::from_str::<CalApiResponse<Schedule>>(&text) {
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
            } else {
                (
                    StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_REQUEST),
                    Json(ResponseAPI::<()>::error(format!("Cal.com error: {}", text))),
                )
                    .into_response()
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(format!("Request failed: {}", e))),
        )
            .into_response(),
    }
}

/// Obtener todos los bookings
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
        .header("cal-api-version", &state.cal_options.api_version)
        .header("Authorization", &state.cal_options.api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text: String = resp.text().await.unwrap_or_default();

            if status.is_success() {
                match serde_json::from_str::<CalBookingsResponse>(&text) {
                    Ok(cal_response) => (
                        StatusCode::OK,
                        Json(ResponseAPI::<Vec<CalBookingPayload>>::success(
                            "Bookings retrieved successfully".to_string(),
                            cal_response.data.bookings,
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
