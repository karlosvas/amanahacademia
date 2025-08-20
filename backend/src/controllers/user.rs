use {
    crate::{
        models::{
            firebase::{
                FirebaseAdminLookupResponse, FirebaseAuthResponse, FirebaseUserInfo, RefreshToken,
                UserAuth, UserAuthentication,
            },
            user::{UserDB, UserRequest},
        },
        services::firebase::handle_firebase_response,
        state::AppState,
    },
    axum::{
        Extension, Json,
        body::Body,
        debug_handler,
        extract::State,
        http::{Response, StatusCode},
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, sync::Arc},
};

// Creación del usuario conn firebase
#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
    // Obtenemos la URL de registro de usuario con Firebase
    let url_register_auth: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        state.firebase.firebase_api_key
    );

    // Creamos el usuario que se va a crear en Firebase Authentication
    let new_user_authentication: UserAuth = UserAuth {
        id_token: None, // No es necesario para crear un usuario
        email: user.email.clone(),
        password: user.password,
        return_secure_token: true,
    };

    // POST:: Crear usuario en Firebase Authentication
    let auth_response: FirebaseAuthResponse = match state
        .firebase_client
        .post(&url_register_auth)
        .json(&new_user_authentication)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseAuthResponse>(response).await {
            Ok(parsed_response) => parsed_response,
            Err((status, error)) => return (status, Json(error)).into_response(),
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Error connecting to Firebase" })),
            )
                .into_response();
        }
    };

    // URL de para crear usuario en la DB
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/user_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, auth_response.local_id, auth_response.id_token
    );

    // Creamos el usuario que se va a crear en la DB
    let user_db: UserDB = UserDB {
        email: auth_response.email.clone(),
        role: user.role,
        subscription_tier: user.subscription_tier,
        permissions: user.permissions,
    };

    // POST:: crear usuario
    match state
        .firebase_client
        .put(&url_firebase_db)
        .json(&user_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => (
            StatusCode::CREATED,
            Json(json!({ "success": true, "body": { "token": auth_response.id_token, "message": "User created successfully" } })),
        )
            .into_response(),
        // Si algo ha fallado avisamos de que el usuario no se pudo crear en la base de datos pero
        // el usuario fue creado en Firebase Auth
        Ok(_) => (
            StatusCode::PARTIAL_CONTENT,
            Json(json!({ "success": false, "warning": "User created in Firebase Auth but error saving profile in database"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "warning": "Error saving profile"})),
        )
            .into_response(),
    }
}

// Login de usuario en Firebase
#[debug_handler]
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(login): Json<UserRequest>,
) -> impl IntoResponse {
    // Construir la URL para la autenticación
    let url_login_firebase: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        state.firebase.firebase_api_key
    );

    // Crear el cuerpo de la solicitud para el login de usuario
    let login_payload: UserAuth = UserAuth {
        id_token: None,
        email: login.email,
        password: login.password,
        return_secure_token: true,
    };

    // Enviar la solicitud a Firebase Auth
    match state
        .firebase_client
        .post(&url_login_firebase)
        .json(&login_payload)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseAuthResponse>(response).await {
            Ok(auth_response) => {
                (StatusCode::OK, Json(json!({ "success": true, "body": { "token": auth_response.id_token, "message": "Login successful" } }))).into_response()
            }
            Err((status, error)) => return (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    }
}

// Actualización de usuario
#[debug_handler]
pub async fn update_user(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
    Json(user_request): Json<UserRequest>,
) -> impl IntoResponse {
    // URL para la actualización de usuario en Firebase
    let url_firebase_auth_update: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:update?key={}",
        state.firebase.firebase_api_key
    );

    // Cuerpo de la solicitud para la actualización de usuario
    let user_payload: UserAuth = UserAuth {
        id_token: Some(id_token.clone()),
        email: user_request.email.clone(), // El email es obligatorio darlo en la request
        password: user_request.password.clone(), // La contraseña es obligatoria darle en la request
        return_secure_token: true,
    };

    // Enviar la solicitud a Firebase Auth
    let _: Response<Body> = match state
        .firebase_client
        .post(&url_firebase_auth_update)
        .json(&user_payload)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseAuthResponse>(response).await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err((status, error)) => return (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    };

    // URL para la base de datos de Firebase
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/user_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, user_claims.sub, id_token
    );

    // Obtener los datos del usuario actual
    let actual_user_db: UserDB = match get_user_data(&user_claims, &id_token, &state).await {
        Some(user_db) => user_db,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Error getting user data" })),
            )
                .into_response();
        }
    };

    // Objeto con características del usuario nuevas
    let user_db: UserDB = UserDB {
        email: user_request.email, // El email es obligatorio darlo en la request
        // Preguntamos si en la request se ha dado un nuevo role, si no lo dejamos como estaba
        role: user_request.role.or(actual_user_db.role),
        // Preguntamos si en la request se ha proporcionado un nuevo subscription_tier, si no lo dejamos como estaba
        subscription_tier: user_request
            .subscription_tier
            .or(actual_user_db.subscription_tier),
        // Preguntamos si en la request se ha dado un nuevo permissions, si no lo dejamos como estaba
        permissions: match (actual_user_db.permissions, user_request.permissions) {
            (Some(mut existing), Some(new)) => {
                existing.extend(new);
                Some(existing)
            }
            (None, Some(new)) => Some(new),
            (Some(existing), None) => Some(existing),
            (None, None) => None,
        },
    };

    // Actualizar en la base de datos
    match state
        .firebase_client
        .put(&url_firebase_db)
        .json(&user_db)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<UserDB>(response).await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({ "success": true, "message": "User updated successfully" })),
            )
                .into_response(),
            Err((status, error)) => (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    }
}

// Eliminar el usuario actualmente autentificado
#[debug_handler]
pub async fn delete_me(
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // URL para eliminar el usuario especificado en Firebase Authentication
    let url_firebase_auth: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:delete?key={}",
        state.firebase.firebase_api_key
    );

    // Obtenemos el usuario de las claims y lo borramos
    // Si falló la eliminación de Auth, retornar error inmediatamente
    let _: Response<Body> = match state
        .firebase_client
        .post(url_firebase_auth) // Fb utiliza POST para eliminar usuarios
        .json(&json!({ "idToken": id_token }))
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseAuthResponse>(response).await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err((status, error)) => (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    };

    // URL de para eliminar usuario en la DB
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/user_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, user_claims.sub, id_token
    );

    // Eliminamos el usuario de Firebase Realtime Database
    match state.firebase_client.delete(url_firebase_db).send().await {
        Ok(response) if response.status().is_success() => (StatusCode::NO_CONTENT).into_response(),
        Ok(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid credential" })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    }
}

// Obtener todos los usuarios
#[debug_handler]
pub async fn get_all_users(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos de la base de datos el usuario actual
    let actual_user_db: UserDB = match get_user_data(&user_claims, &id_token, &state).await {
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
        "https://{}.firebasedatabase.app/user_profiles.json?auth={}",
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
            Ok(response) => {
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

    // URL de los datos de Firebase Admin
    let url_firebase_admin: String = format!(
        // Obtenemos la información del usuario de Firebase Realtime Database
        "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
        state.firebase.firebase_api_key
    );

    // Realizamos la petición a Firebase Admin para obtener la información del usuario
    let user_data_auth: FirebaseAdminLookupResponse = match state
        .firebase_client
        .post(&url_firebase_admin)
        .json(&json!({ "idToken": id_token }))
        .send()
        .await
    {
        Ok(response) => {
            match handle_firebase_response::<FirebaseAdminLookupResponse>(response).await {
                Ok(users) => users,
                Err((status, error)) => {
                    return (status, Json(error)).into_response();
                }
            }
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
            "body": { "users": {
                "firebase": user_data_auth,
                "database": user_data_db
            }, "message": "Users retrieved successfully" }
        })),
    )
        .into_response()
}

// Refrescar el token
#[debug_handler]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(refresh_token): Json<RefreshToken>,
) -> impl IntoResponse {
    // URL de Firebase para refrescar el token
    let url_firebase_auth_refresh_token: String = format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        state.firebase.firebase_api_key
    );

    // Actualizamos el neuvo token, no hace falta devolber nada
    match state
        .firebase_client
        .post(&url_firebase_auth_refresh_token)
        .json(&refresh_token)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseAuthResponse>(response).await {
            Ok(auth_response) => (
                StatusCode::OK,
                Json(json!({ "success": true, "body": auth_response })),
            )
                .into_response(),
            Err(_) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid refresh token" })),
            )
                .into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error refreshing token" })),
        )
            .into_response(),
    }
}

// Obtener datos del usuario según su sesión
pub async fn get_user_data(
    user_claims: &UserAuthentication,
    id_token: &str,
    state: &Arc<AppState>,
) -> Option<UserDB> {
    // URL de Firebase Realtime Database para obtener los datos del usuario
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/user_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, user_claims.sub, id_token
    );

    // Realizamos la petición a Firebase Realtime Database
    match state.firebase_client.get(url_firebase_db).send().await {
        Ok(response) => match handle_firebase_response::<UserDB>(response).await {
            Ok(user) => Some(user),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
