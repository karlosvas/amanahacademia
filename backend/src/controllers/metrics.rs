use {
    crate::{
        models::{
            metrics::{GAResponse, GAToken},
            response::ResponseAPI,
            state::AppState,
        },
        services::metrics::parse_ga_response,
    },
    axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse},
    reqwest::{Error, Response},
    serde_json::Value,
    std::{
        result::Result::{Err, Ok},
        sync::Arc,
    },
};

/// Función auxiliar para ejecutar consultas a Google Analytics
async fn fetch_ga_metrics(
    state: Arc<AppState>,
    token_ga: String,
    body: Value,
    success_message: String,
) -> impl IntoResponse {
    let response: Result<Response, Error> = state
        .ga_options
        .client
        .post(format!(
            "https://analyticsdata.googleapis.com/v1beta/properties/{}:runReport",
            state.ga_options.property_id
        ))
        .bearer_auth(&token_ga)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    let ga_response: GAResponse = match parse_ga_response(response).await {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(e.to_string())),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<GAResponse>::success(
            success_message,
            ga_response,
        )),
    )
        .into_response()
}

/// Controlador para obtener métricas de usuarios desde Google Analytics
pub async fn get_user_metrics(
    State(state): State<Arc<AppState>>,
    Extension(GAToken(token_ga)): Extension<GAToken>,
) -> impl IntoResponse {
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

    fetch_ga_metrics(state, token_ga, body, "Metrics retrieved successfully".to_string()).await
}

/// Controlador para obtener métricas de artículos desde Google Analytics
pub async fn get_article_metrics(
    State(state): State<Arc<AppState>>,
    Extension(GAToken(token_ga)): Extension<GAToken>,
) -> impl IntoResponse {
    let body: Value = serde_json::json!({
        "dateRanges": [{"startDate": "365daysAgo", "endDate": "today"}],
        "dimensions": [
            {"name": "eventName"},
            {"name": "yearMonth"}
        ],
        "metrics": [
            {"name": "eventCount"},
            {"name": "totalUsers"}
        ],
        "dimensionFilter": {
            "filter": {
                "fieldName": "eventName",
                "stringFilter": {
                    "matchType": "EXACT",
                    "value": "article_click"
                }
            }
        }
    });

    fetch_ga_metrics(state, token_ga, body, "Metrics of articles retrieved successfully".to_string()).await
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

    fetch_ga_metrics(state, token_ga, body, "Class booking metrics retrieved successfully".to_string()).await
}
