use {
    crate::{
        models::{response::ResponseAPI, sourvey::Survey, state::AppState},
        services::firebase::handle_firebase_response,
    },
    axum::{
        Extension, Json, debug_handler,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    std::{collections::HashMap, sync::Arc},
    tracing::instrument,
    uuid::Uuid,
};

// Crear encuesta
#[debug_handler]
#[instrument(skip(state, id_token, survey), fields(operation = "create_survey"))]
pub async fn create_survey(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Json(mut survey): Json<Survey>,
) -> impl IntoResponse {
    // Si no llega id, generamos uno para mantener consistencia en el modelo
    if survey.id.trim().is_empty() {
        survey.id = Uuid::new_v4().to_string();
    }

    let url_firebase_db: String = format!(
        "{}/surveys/{}.json?auth={}",
        state.firebase_options.firebase_database_url, survey.id, id_token
    );

    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&survey)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Survey>(response).await {
            Ok(created) => (
                StatusCode::CREATED,
                Json(ResponseAPI::<Survey>::success(
                    "Survey created successfully".to_string(),
                    created,
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Error connecting to Firebase".to_string(),
            )),
        )
            .into_response(),
    }
}

// Obtener resultados de encuestas
#[debug_handler]
#[instrument(skip(state, id_token), fields(operation = "get_survey_results", survey_id = %survey_id))]
pub async fn get_survey_results(
    Path(survey_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
) -> impl IntoResponse {
    let url_firebase_db: String = format!(
        "{}/surveys.json?auth={}",
        state.firebase_options.firebase_database_url, id_token
    );

    let surveys_map: HashMap<String, Survey> = match state
        .firebase_options
        .firebase_client
        .get(&url_firebase_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => match response.text().await {
            Ok(response_text) => {
                if response_text.trim().is_empty() || response_text.trim() == "null" {
                    HashMap::new()
                } else {
                    match serde_json::from_str::<HashMap<String, Survey>>(&response_text) {
                        Ok(value) => value,
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ResponseAPI::<()>::error(
                                    "Error parsing surveys data".to_string(),
                                )),
                            )
                                .into_response();
                        }
                    }
                }
            }
            Err(_) => HashMap::new(),
        },
        Ok(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error retrieving surveys from database".to_string(),
                )),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error connecting to Firebase".to_string(),
                )),
            )
                .into_response();
        }
    };

    let data: Vec<Survey> = if survey_id == "latest" {
        surveys_map.into_values().collect()
    } else {
        match surveys_map.get(&survey_id) {
            Some(survey) => vec![survey.clone()],
            None => Vec::new(),
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<Vec<Survey>>::success(
            "Survey results fetched successfully".to_string(),
            data,
        )),
    )
        .into_response()
}

// Obtener todos los resultados de encuestas
#[debug_handler]
#[instrument(skip(state, id_token), fields(operation = "get_all_survey_results"))]
pub async fn get_all_survey_results(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
) -> impl IntoResponse {
    let url_firebase_db: String = format!(
        "{}/surveys.json?auth={}",
        state.firebase_options.firebase_database_url, id_token
    );

    let surveys_map: HashMap<String, Survey> = match state
        .firebase_options
        .firebase_client
        .get(&url_firebase_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => match response.text().await {
            Ok(response_text) => {
                if response_text.trim().is_empty() || response_text.trim() == "null" {
                    HashMap::new()
                } else {
                    match serde_json::from_str::<HashMap<String, Survey>>(&response_text) {
                        Ok(value) => value,
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ResponseAPI::<()>::error(
                                    "Error parsing surveys data".to_string(),
                                )),
                            )
                                .into_response();
                        }
                    }
                }
            }
            Err(_) => HashMap::new(),
        },
        Ok(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error retrieving surveys from database".to_string(),
                )),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error connecting to Firebase".to_string(),
                )),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(ResponseAPI::<Vec<Survey>>::success(
            "All survey results fetched successfully".to_string(),
            surveys_map.into_values().collect(),
        )),
    )
        .into_response()
}
