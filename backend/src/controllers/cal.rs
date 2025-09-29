use {
    crate::{models::response::ResponseAPI, state::AppState},
    axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    std::sync::Arc,
};

pub async fn confirm_booking(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let url_cal = format!("{}/bookings/{}/confirm", state.cal_options.base_url, id);

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
