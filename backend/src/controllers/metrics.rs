use {
    crate::models::{
        metrics::{GAErrorResponse, GAResponse, GAToken},
        response::ResponseAPI,
        state::AppState,
    },
    axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse},
    reqwest::Response,
    serde_json::Value,
    std::sync::Arc,
};

/// Controlador para obtener métricas de usuarios desde Google Analytics
pub async fn get_user_metrics(
    State(state): State<Arc<AppState>>,
    Extension(GAToken(token_ga)): Extension<GAToken>,
) -> impl IntoResponse {
    // El usuario es admin, continuar con la lógica de métricas
    let body: Value = serde_json::json!({
        "dateRanges": [{"startDate": "365daysAgo", "endDate": "today"}],
        "dimensions": [{"name": "yearMonth"}],
        "metrics": [
            {"name": "activeUsers"},
            {"name": "totalUsers"},
            {"name": "newUsers"},
            {"name": "sessions"},
            {"name": "engagedSessions"},
            {"name": "averageSessionDuration"},
            {"name": "bounceRate"},
            {"name": "sessionsPerUser"}
        ]
    });

    let response: Response = match state
        .ga_options
        .client
        .post(format!(
            "https://analyticsdata.googleapis.com/v1beta/properties/{}:runReport",
            state.ga_options.property_id
        ))
        .bearer_auth(token_ga)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to fetch metrics: {}",
                    e
                ))),
            )
                .into_response();
        }
    };

    // Check if the response was successful
    if !response.status().is_success() {
        // Try to parse as an error response from Google Analytics
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        if let Ok(ga_error) = serde_json::from_str::<GAErrorResponse>(&error_text) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseAPI::<()>::error(format!(
                    "Google Analytics API error: {} ({})",
                    ga_error.error.message, ga_error.error.status
                ))),
            )
                .into_response();
        }

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(format!(
                "Google Analytics API returned error: {}",
                error_text
            ))),
        )
            .into_response();
    }

    // Parse the successful response
    let ga_response: GAResponse = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to parse metrics response: {}",
                    e
                ))),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<GAResponse>::success(
            "Metrics retrieved successfully".to_string(),
            ga_response,
        )),
    )
        .into_response()
}

/// Controlador para obtener métricas de artículos desde Google Analytics
pub async fn get_article_metrics(
    State(state): State<Arc<AppState>>,
    Extension(GAToken(token_ga)): Extension<GAToken>,
) -> impl IntoResponse {
    // El usuario es admin, continuar con la lógica de métricas
    let body: Value = serde_json::json!({
        "dateRanges": [{"startDate": "365daysAgo", "endDate": "today"}],
        "dimensions": [{"name": "pagePath"}],
        "metrics": [
            {"name": "activeUsers"},
            {"name": "totalUsers"},
            {"name": "newUsers"},
            {"name": "sessions"},
            {"name": "engagedSessions"},
            {"name": "averageSessionDuration"},
            {"name": "bounceRate"},
            {"name": "sessionsPerUser"}
        ],
        "dimensionFilter": {
            "filter": {
                "fieldName": "pagePath",
                "stringFilter": {
                    "matchType": "CONTAINS",
                    "value": "/articles/"
                }
            }
        }
    });

    let response: Response = match state
        .ga_options
        .client
        .post(format!(
            "https://analyticsdata.googleapis.com/v1beta/properties/{}:runReport",
            state.ga_options.property_id
        ))
        .bearer_auth(token_ga)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to fetch metrics: {}",
                    e
                ))),
            )
                .into_response();
        }
    };

    if !response.status().is_success() {
        // Try to parse as an error response from Google Analytics
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        if let Ok(ga_error) = serde_json::from_str::<GAErrorResponse>(&error_text) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseAPI::<()>::error(format!(
                    "Google Analytics API error: {} ({})",
                    ga_error.error.message, ga_error.error.status
                ))),
            )
                .into_response();
        }

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(format!(
                "Google Analytics API returned error: {}",
                error_text
            ))),
        )
            .into_response();
    }

    let ga_response: GAResponse = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to parse metrics response: {}",
                    e
                ))),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<GAResponse>::success(
            "Article metrics retrieved successfully".to_string(),
            ga_response,
        )),
    )
        .into_response()
}

/// Controlador para obtener métricas de reservas de clases
pub async fn get_class_metrics(
    State(state): State<Arc<AppState>>,
    Extension(GAToken(token_ga)): Extension<GAToken>,
) -> impl IntoResponse {
    let body: Value = serde_json::json!({
        "dateRanges": [{"startDate": "365daysAgo", "endDate": "today"}],
        "dimensions": [
            {"name": "yearMonth"},
            {"name": "eventName"}
        ],
        "metrics": [
            {"name": "eventCount"}
        ],
        "dimensionFilter": {
            "filter": {
                "fieldName": "eventName",
                "stringFilter": {
                    "value": "class_booking"
                }
            }
        }
    });

    let response: Response = match state
        .ga_options
        .client
        .post(format!(
            "https://analyticsdata.googleapis.com/v1beta/properties/{}:runReport",
            state.ga_options.property_id
        ))
        .bearer_auth(token_ga)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to fetch metrics: {}",
                    e
                ))),
            )
                .into_response();
        }
    };

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        if let Ok(ga_error) = serde_json::from_str::<GAErrorResponse>(&error_text) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseAPI::<()>::error(format!(
                    "Google Analytics API error: {} ({})",
                    ga_error.error.message, ga_error.error.status
                ))),
            )
                .into_response();
        }

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(format!(
                "Google Analytics API returned error: {}",
                error_text
            ))),
        )
            .into_response();
    }

    let ga_response: GAResponse = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(format!(
                    "Failed to parse metrics response: {}",
                    e
                ))),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<GAResponse>::success(
            "Class booking metrics retrieved successfully".to_string(),
            ga_response,
        )),
    )
        .into_response()
}
