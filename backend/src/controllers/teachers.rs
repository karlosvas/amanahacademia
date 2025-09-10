use {
    crate::{
        models::{
            firebase::{FirebaseAuthResponse, UserAuthentication},
            user::{UserDB, UserRequest},
        },
        services::firebase::handle_firebase_response,
        state::AppState,
    },
    axum::{
        Extension, Json, debug_handler,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, sync::Arc},
};

// Obtener teacher por ID
#[debug_handler]
pub async fn get_teacher(
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Lógica para obtener la información del profesor por su ID
    // Puedes usar `state` para acceder al estado de la aplicación si es necesario
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/teacher_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, id, id_token
    );

    // Realizamos la petición a Firebase Realtime Database
    let user: Option<UserDB> = match state.firebase_client.get(url_firebase_db).send().await {
        Ok(response) => {
            println!("Firebase response: {:?}", response);
            match handle_firebase_response::<UserDB>(response).await {
                Ok(user) => Some(user),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };

    match user {
        Some(user) => (
            axum::http::StatusCode::OK,
            Json(json!({ "success": true, "user": user })),
        )
            .into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "error": "Teacher not found" })),
        )
            .into_response(),
    }
}

// Crear nuevo teacher
#[debug_handler]
pub async fn create_teacher(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
    // Obtenemos el auth de firebase auth
    let auth_response: FirebaseAuthResponse =
        match crate::controllers::users::get_user_data_auth(&state, &user).await {
            Some(auth_response) => auth_response,
            None => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(
                        json!({ "success": false, "error": "Error obtaining Firebase auth token" }),
                    ),
                );
            }
        };

    // URL de para crear usuario en la DB
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/teacher_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, auth_response.local_id, auth_response.id_token
    );

    // Creamos el usuario que se va a crear en la DB
    let user_db: UserDB = UserDB {
        email: auth_response.email.clone(),
        role: Some(match user.role {
            Some(role) => role.to_string(),
            None => "teacher".to_string(),
        }),
        subscription_tier: user.subscription_tier,
        permissions: user.permissions,
    };

    // Hacemos GET para comprobar si ya existia el usuario en la base de datos
    let url_check_db = format!(
        "https://{}.firebasedatabase.app/teacher_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, auth_response.local_id, auth_response.id_token
    );
    let exists = match state.firebase_client.get(&url_check_db).send().await {
        Ok(resp) => resp.text().await.unwrap_or_default() != "null",
        Err(_) => false,
    };
    if exists {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "body": {
                    "token": auth_response.id_token,
                    "message": "User already exists in database"
                }
            })),
        );
    }

    // POST:: Crear usuario en FB_DATABASE
    match state
        .firebase_client
        .put(&url_firebase_db)
        .json(&user_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => (
            StatusCode::CREATED,
            Json(
                json!({ "success": true, "body": { "token": auth_response.id_token, "message": "User created successfully" } }),
            ),
        ),
        Ok(_) => (
            StatusCode::PARTIAL_CONTENT,
            Json(
                json!({ "success": false, "warning": "User created in Firebase Auth but error saving profile in database"}),
            ),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "warning": "Error saving profile"})),
        ),
    }
}

// Obtener todos los teachers
#[debug_handler]
pub async fn get_all_teachers(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos de la base de datos el usuario actual
    let actual_user_db: UserDB =
        match crate::controllers::users::get_user_data_db(&user_claims, &id_token, &state).await {
            Some(user_db) => user_db,
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Error getting user data" })),
                )
                    .into_response();
            }
        };

    // Solo podemos obtener todos los usuarios si es administrador
    if actual_user_db.role != Some("admin".to_string()) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "You do not have permission to access this resource" })),
        )
            .into_response();
    }

    // URL para obtener todos los usuarios de Firebase Realtime Database
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/teacher_profiles.json?auth={}",
        state.firebase.firebase_project_id, id_token
    );

    // Realizamos la petición a Firebase Realtime Database para obtener todos los usuarios
    let user_data_db: HashMap<String, UserDB> =
        match state.firebase_client.get(&url_firebase_db).send().await {
            Ok(response) if response.status().is_success() => match response.text().await {
                Ok(response_text) => {
                    if response_text.trim().is_empty() || response_text.trim() == "null" {
                        HashMap::new()
                    } else {
                        match serde_json::from_str::<HashMap<String, UserDB>>(&response_text) {
                            Ok(value) => value,
                            Err(_) => {
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(json!({ "error": "Error parsing database users data" })),
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
                    Json(json!({ "error": "Error retrieving users from database" })),
                )
                    .into_response();
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Error connecting to Firebase" })),
                )
                    .into_response();
            }
        };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "body": {
                "teacher_profile": user_data_db,
                "message": "Users retrieved successfully"
            }
        })),
    )
        .into_response()
}
