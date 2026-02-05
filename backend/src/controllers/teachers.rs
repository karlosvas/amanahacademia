use {
    crate::{
        models::{response::ResponseAPI, state::AppState, teacher::Teacher},
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
};

// Obtener teacher por ID
#[debug_handler]
#[instrument(skip(state, id_token))]
pub async fn get_teacher(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Lógica para obtener la información del profesor por su ID, creamos la url a el recurso y nos autentificamos.
    let url_firebase_db: String = format!(
        "{}/teacher_profiles/{}.json?auth={}",
        state.firebase_options.firebase_database_url, id, id_token
    );

    // Realizamos la petición a Firebase Realtime Database
    let teacher: Option<Teacher> = match state
        .firebase_options
        .firebase_client
        .get(url_firebase_db)
        .send()
        .await
    {
        Ok(response) => (handle_firebase_response::<Teacher>(response).await).ok(),
        Err(_) => None,
    };

    match teacher {
        Some(teacher) => (
            StatusCode::OK,
            Json(ResponseAPI::<Teacher>::success(
                "success".to_string(),
                teacher,
            ))
            .into_response(),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(ResponseAPI::<()>::error("Teacher not found".to_string())).into_response(),
        ),
    }
}

// Crear nuevo teacher
#[debug_handler]
#[instrument(skip(state, id_token))]
pub async fn create_teacher(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Json(teacher): Json<Teacher>,
) -> impl IntoResponse {
    // URL de para crear usuario en la DB
    let url_firebase_db: String = format!(
        "{}/teacher_profiles.json?auth={}",
        state.firebase_options.firebase_database_url, id_token
    );

    // POST:: Crear profesor en FB_DATABASE
    match state
        .firebase_options
        .firebase_client
        .post(&url_firebase_db)
        .json(&teacher)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<HashMap<String, String>>(response).await {
            Ok(parsed_response) => (
                StatusCode::CREATED,
                Json(ResponseAPI::success(
                    "Teacher created successfully".to_string(),
                    parsed_response,
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error("Error saving profile".to_string())),
        )
            .into_response(),
    }
}

// Mustra todos los profesores
#[debug_handler]
#[instrument(skip(state))]
pub async fn get_all_teachers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // URL para obtener todos los usuarios de Firebase Realtime Database
    let url_firebase_db: String = format!(
        "{}/teacher_profiles.json",
        state.firebase_options.firebase_database_url
    );

    // Realizamos la petición a Firebase Realtime Database para obtener todos los usuarios
    let user_data_db: HashMap<String, Teacher> = match state
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
                    match serde_json::from_str::<HashMap<String, Teacher>>(&response_text) {
                        Ok(value) => value,
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ResponseAPI::<()>::error(
                                    "Error parsing database users data".to_string(),
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
                    "Error retrieving users from database".to_string(),
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
        Json(ResponseAPI::<HashMap<String, Teacher>>::success(
            "Users retrieved successfully".to_string(),
            user_data_db,
        )),
    )
        .into_response()
}

// Elimina un profesor
#[debug_handler]
#[instrument(skip(state, id_token))]
pub async fn delete_teacher(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Lógica para eliminar un profesor por su ID
    let url_firebase_db: String = format!(
        "{}/teacher_profiles/{}.json?auth={}",
        state.firebase_options.firebase_database_url, id, id_token
    );

    match state
        .firebase_options
        .firebase_client
        .delete(&url_firebase_db)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<()>(response).await {
            Ok(_) => (StatusCode::NO_CONTENT, Json(ResponseAPI::success_no_data())),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error.to_string()))),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Error connecting to Firebase".to_string(),
            )),
        ),
    }
}
