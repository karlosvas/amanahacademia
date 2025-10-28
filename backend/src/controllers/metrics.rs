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

    let response: Result<Response, Error> = state
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
        .await;

    let ga_response: GAResponse = match parse_ga_response(response).await {
        // Si la función interna tiene éxito, asigna el valor.
        Ok(parsed_data) => parsed_data,
        // Si la función interna devuelve Err(e), crea y retorna la respuesta de error HTTP
        Err(e) => {
            return (
                // Puedes elegir un StatusCode adecuado, BAD_GATEWAY o INTERNAL_SERVER_ERROR son comunes.
                StatusCode::INTERNAL_SERVER_ERROR,
                // Usa tu estructura de respuesta de API para el error
                Json(ResponseAPI::<()>::error(e.to_string())),
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

    let response: Result<Response, Error> = state
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
        .await;

    let ga_response: GAResponse = match parse_ga_response(response).await {
        // Si la función interna tiene éxito, asigna el valor.
        Ok(parsed_data) => parsed_data,
        // Si la función interna devuelve Err(e), crea y retorna la respuesta de error HTTP
        Err(e) => {
            return (
                // Puedes elegir un StatusCode adecuado, BAD_GATEWAY o INTERNAL_SERVER_ERROR son comunes.
                StatusCode::INTERNAL_SERVER_ERROR,
                // Usa tu estructura de respuesta de API para el error
                Json(ResponseAPI::<()>::error(e.to_string())),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<GAResponse>::success(
            "Metrics of articles retrieved successfully".to_string(),
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

    let response: Result<Response, Error> = state
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
        .await;

    let ga_response: GAResponse = match parse_ga_response(response).await {
        // Si la función interna tiene éxito, asigna el valor.
        Ok(parsed_data) => parsed_data,
        // Si la función interna devuelve Err(e), crea y retorna la respuesta de error HTTP
        Err(e) => {
            return (
                // Puedes elegir un StatusCode adecuado, BAD_GATEWAY o INTERNAL_SERVER_ERROR son comunes.
                StatusCode::INTERNAL_SERVER_ERROR,
                // Usa tu estructura de respuesta de API para el error
                Json(ResponseAPI::<()>::error(e.to_string())),
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
