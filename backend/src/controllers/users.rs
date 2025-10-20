use {
    crate::{
        models::{
            firebase::{
                FirebaseAdminLookupResponse, FirebaseAuthResponse, RefreshToken, UserAuth,
                UserAuthentication, UserMerged,
            },
            response::ResponseAPI,
            user::{Provider, UserDB, UserRequest},
        },
        services::firebase::handle_firebase_response,
        state::AppState,
    },
    axum::{
        Extension, Json,
        body::Body,
        debug_handler,
        extract::{State, rejection::JsonRejection},
        http::{Response, StatusCode},
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, sync::Arc},
    tracing::instrument,
};

// Creación del usuario conn firebase
#[debug_handler]
#[instrument(
    skip(state, payload),
    fields(
        email = tracing::field::Empty
    )
)]
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    payload: Result<Json<UserRequest>, JsonRejection>,
) -> impl IntoResponse {
    // Handle JSON deserialization errors
    let Json(user) = match payload {
        Ok(json) => json,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResponseAPI::<()>::error(format!(
                    "Invalid JSON payload: {}",
                    err
                ))),
            )
                .into_response();
        }
    };

    // Record email in tracing span
    tracing::Span::current().record("email", &user.email.as_str());
    // Comprobamos si quiere hacer cosas que solo podria hacer un admin como tener un rol, o asiganr permisos o tier de subscripción
    match &user.role {
        Some(role) if role == "admin" => {
            return (
                StatusCode::FORBIDDEN,
                Json(ResponseAPI::<()>::error(
                    "You do not have permission to assign this role".to_string(),
                )),
            )
                .into_response();
        }
        _ => {}
    }
    if user.permissions.as_ref().is_some() || user.subscription_tier.as_ref().is_some() {
        return (
            StatusCode::FORBIDDEN,
            Json(ResponseAPI::<()>::error(
                "You do not have permission to assign these permissions or subscription tier"
                    .to_string(),
            )),
        )
            .into_response();
    }

    // Obtenemos la URL de registro de usuario con Firebase
    let url_register_auth: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        state.firebase_options.firebase_api_key
    );

    // Creamos el usuario que se va a crear en Firebase Authentication
    let new_user_authentication: UserAuth = UserAuth {
        id_token: None, // No es necesario para crear un usuario
        email: user.email.clone(),
        password: user.password.clone(),
        return_secure_token: true,
        display_name: user.name.clone(),
    };

    let auth_response: FirebaseAuthResponse = if user.provider == Provider::Email {
        // POST:: Crear usuario en Firebase Authentication
        match state
            .firebase_options
            .firebase_client
            .post(&url_register_auth)
            .json(&new_user_authentication)
            .send()
            .await
        {
            Ok(response) => {
                match handle_firebase_response::<FirebaseAuthResponse>(response).await {
                    Ok(parsed_response) => parsed_response,
                    Err((status, error)) => {
                        println!("Error creating user in Firebase: {}", error);
                        return (status, Json(ResponseAPI::<()>::error(error))).into_response();
                    }
                }
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
        }
    } else {
        // For social providers, we expect an id_token to be provided
        match &user.id_token {
            Some(token) => {
                // Verify the token by looking up the user
                let url_firebase_admin = format!(
                    "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
                    state.firebase_options.firebase_api_key
                );

                match state
                    .firebase_options
                    .firebase_client
                    .post(&url_firebase_admin)
                    .json(&json!({ "idToken": token }))
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        match handle_firebase_response::<FirebaseAdminLookupResponse>(response)
                            .await
                        {
                            Ok(user_data) => {
                                // User already exists, return their data
                                FirebaseAuthResponse {
                                    id_token: token.clone(),
                                    email: user_data.users[0].email.clone().unwrap_or_default(),
                                    refresh_token: "".to_string(),
                                    expires_in: "".to_string(),
                                    local_id: user_data.users[0].local_id.clone(),
                                    registered: Some(true),
                                }
                            }
                            Err((status, error)) => {
                                return (status, Json(error)).into_response();
                            }
                        }
                    }
                    _ => {
                        return (
                            StatusCode::UNAUTHORIZED,
                            Json(ResponseAPI::<()>::error(
                                "Invalid token provided".to_string(),
                            )),
                        )
                            .into_response();
                    }
                }
            }
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error(
                        "Token is required for social provider authentication".to_string(),
                    )),
                )
                    .into_response();
            }
        }
    };

    create_user_in_db(
        &state,
        &auth_response.id_token,
        &auth_response.local_id,
        &user,
        &auth_response.email,
    )
    .await
    .into_response()
}

pub async fn create_user_in_db(
    state: &Arc<AppState>,
    id_token: &str,
    user_id: &str,
    user: &UserRequest,
    email: &str,
) -> impl IntoResponse {
    // URL de para crear usuario en la DB
    let url_firebase_db: String = format!(
        "{}/user_profiles/{}.json?auth={}",
        state.firebase_options.firebase_database_url, user_id, id_token
    );

    // Creamos el usuario que se va a crear en la DB
    let user_db: UserDB = UserDB {
        email: email.to_string(),
        first_free_class: user.first_free_class,
        role: Some(match &user.role {
            Some(role) => role.to_string(),
            None => "student".to_string(),
        }),
        subscription_tier: user.subscription_tier.clone(),
        permissions: user.permissions.clone(),
    };

    // POST:: crear usuario
    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&user_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => (
            StatusCode::CREATED,
            Json(ResponseAPI::<String>::success(
                "User created successfully".to_string(),
                id_token.to_string(),
            )),
        )
            .into_response(),
        // Si algo ha fallado avisamos de que el usuario no se pudo crear en la base de datos pero
        // el usuario fue creado en Firebase Auth
        Ok(_) => (
            StatusCode::PARTIAL_CONTENT,
            Json(ResponseAPI::<()>::error(
                "User created in Firebase Auth but error saving profile in database".to_string(),
            )),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error("Error saving profile".to_string())),
        )
            .into_response(),
    }
}

// Login de usuario en Firebase
#[debug_handler]
#[instrument(
    skip(state, user_request),
    fields(
        email = %user_request.email
    )
)]
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(user_request): Json<UserRequest>,
) -> impl IntoResponse {
    // Handle different providers
    let auth_response: FirebaseAuthResponse = if user_request.provider == Provider::Email {
        // Construir la URL para la autenticación con email/password
        let url_login_firebase: String = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
            state.firebase_options.firebase_api_key
        );

        // Crear el cuerpo de la solicitud para el login de usuario
        let login_payload: UserAuth = UserAuth {
            id_token: None,
            email: user_request.email.clone(),
            password: user_request.password.clone(),
            return_secure_token: true,
            display_name: None,
        };

        // Enviar la solicitud a Firebase Auth
        match state
            .firebase_options
            .firebase_client
            .post(&url_login_firebase)
            .json(&login_payload)
            .send()
            .await
        {
            Ok(response) => {
                match handle_firebase_response::<FirebaseAuthResponse>(response).await {
                    Ok(auth_response) => auth_response,
                    Err((status, error)) => {
                        return (status, Json(ResponseAPI::<()>::error(error))).into_response();
                    }
                }
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
        }
    } else {
        // For social providers (Google), verify the id_token
        match &user_request.id_token {
            Some(token) => {
                let url_firebase_admin = format!(
                    "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
                    state.firebase_options.firebase_api_key
                );

                match state
                    .firebase_options
                    .firebase_client
                    .post(&url_firebase_admin)
                    .json(&json!({ "idToken": token }))
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        match handle_firebase_response::<FirebaseAdminLookupResponse>(response)
                            .await
                        {
                            Ok(user_data) => FirebaseAuthResponse {
                                id_token: token.clone(),
                                email: user_data.users[0].email.clone().unwrap_or_default(),
                                refresh_token: "".to_string(),
                                expires_in: "".to_string(),
                                local_id: user_data.users[0].local_id.clone(),
                                registered: Some(true),
                            },
                            Err((status, error)) => {
                                return (status, Json(error)).into_response();
                            }
                        }
                    }
                    _ => {
                        return (
                            StatusCode::UNAUTHORIZED,
                            Json(ResponseAPI::<()>::error(
                                "Invalid token provided".to_string(),
                            )),
                        )
                            .into_response();
                    }
                }
            }
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error(
                        "Token is required for social provider authentication".to_string(),
                    )),
                )
                    .into_response();
            }
        }
    };

    if user_request.provider == Provider::Google {
        // Verifiamos si existia en la base de datos si existe no hacemos nada y si no existe lo creamos
        match get_user_data_db(
            &UserAuthentication {
                sub: auth_response.local_id.clone(),
                iss: String::new(),
                aud: String::new(),
                iat: 0,
                exp: 0,
                email: Some(auth_response.email.clone()),
                email_verified: None,
                name: None,
                picture: None,
                auth_time: 0,
                user_id: auth_response.local_id.clone(),
                firebase: None,
                phone_number: None,
                provider_id: None,
            },
            &auth_response.id_token,
            &state,
        )
        .await
        {
            Some(_) => {}
            None => {
                // Devolbemos un error avisando de que el usuario deveria registrarse
                return (
                    StatusCode::NOT_FOUND,
                    Json(ResponseAPI::<()>::error(
                        "User does not exist, please register".to_string(),
                    )),
                )
                    .into_response();
            }
        }
    }

    (
        StatusCode::OK,
        Json(ResponseAPI::<String>::success(
            "Login successful".to_string(),
            auth_response.id_token,
        )),
    )
        .into_response()
}

// Actualización de usuario
#[debug_handler]
#[instrument(
    skip(state, user_claims, id_token, user_request),
    fields(
        user_id = %user_claims.sub,
        email = %user_request.email,
        operation = "update_user"
    )
)]
pub async fn update_user(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
    Json(user_request): Json<UserRequest>,
) -> impl IntoResponse {
    // URL para la actualización de usuario en Firebase
    let url_firebase_auth_update: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:update?key={}",
        state.firebase_options.firebase_api_key
    );

    // Cuerpo de la solicitud para la actualización de usuario
    let user_payload: UserAuth = UserAuth {
        id_token: Some(id_token.clone()), // <-- Aquí va el token de sesión del usuario
        email: user_request.email.clone(), // El email es obligatorio darlo en la request
        password: user_request.password.clone(), // La contraseña es obligatoria darle en la request
        return_secure_token: true,
        display_name: user_request.name.clone(),
    };

    // Enviar la solicitud a Firebase Auth
    let _: Response<Body> = match state
        .firebase_options
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
            Json(ResponseAPI::<()>::error(
                "Error connecting to Firebase".to_string(),
            )),
        )
            .into_response(),
    };

    // URL para la base de datos de Firebase
    let url_firebase_db: String = format!(
        "{}/user_profiles/{}.json?auth={}",
        state.firebase_options.firebase_database_url, user_claims.sub, id_token
    );

    // Obtener los datos del usuario actual
    let actual_user_db: UserDB = match get_user_data_db(&user_claims, &id_token, &state).await {
        Some(user_db) => user_db,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error getting user data".to_string(),
                )),
            )
                .into_response();
        }
    };

    // Objeto con características del usuario nuevas
    let user_db: UserDB = UserDB {
        email: user_request.email, // El email es obligatorio darlo en la request
        first_free_class: user_request.first_free_class, // Mantenemos el valor que venga en la request
        // Preguntamos si en la request se ha dado un nuevo role, si no lo dejamos como estaba
        role: user_request
            .role
            .map(|r| r.to_string())
            .or_else(|| actual_user_db.role.map(|r| r.to_string())),
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
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&user_db)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<UserDB>(response).await {
            Ok(user) => (
                StatusCode::OK,
                Json(ResponseAPI::<UserDB>::success(
                    "User updated successfully".to_string(),
                    user,
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(error)).into_response(),
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

// Obtener todos los usuarios
#[debug_handler]
#[instrument(
    skip(state, user_claims, id_token),
    fields(
        user_id = %user_claims.sub
    )
)]
pub async fn get_all_users(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos de la base de datos el usuario actual
    let actual_user_db: UserDB = match get_user_data_db(&user_claims, &id_token, &state).await {
        Some(user_db) => user_db,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error getting user data".to_string(),
                )),
            )
                .into_response();
        }
    };

    // Solo podemos obtener todos los usuarios si es administrador
    if actual_user_db.role != Some("admin".to_string()) {
        println!("User is not admin, denying access");
        return (
            StatusCode::FORBIDDEN,
            Json(ResponseAPI::<()>::error(
                "You do not have permission to access this resource".to_string(),
            )),
        )
            .into_response();
    } else {
        println!("User is admin, proceeding...");
    }

    // URL para obtener todos los usuarios de Firebase Realtime Database
    let url_firebase_db: String = format!(
        "{}/user_profiles.json?auth={}",
        state.firebase_options.firebase_database_url, id_token
    );

    // Realizamos la petición a Firebase Realtime Database para obtener todos los usuarios
    let user_data_db: HashMap<String, UserDB> = match state
        .firebase_options
        .firebase_client
        .get(&url_firebase_db)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(response_text) => {
                        if response_text.trim().is_empty() || response_text.trim() == "null" {
                            HashMap::new()
                        } else {
                            match serde_json::from_str::<HashMap<String, UserDB>>(&response_text) {
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
                }
            } else {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Error retrieving users from database".to_string(),
                    )),
                )
                    .into_response();
            }
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

    // URL de los datos de Firebase Admin
    let url_firebase_admin: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
        state.firebase_options.firebase_api_key
    );

    // Realizamos la petición a Firebase Admin para obtener la información del usuario
    let user_data_auth: FirebaseAdminLookupResponse = match state
        .firebase_options
        .firebase_client
        .post(&url_firebase_admin)
        .json(&json!({ "idToken": id_token }))
        .send()
        .await
    {
        Ok(response) => {
            println!("Firebase Admin response status: {}", response.status());
            match handle_firebase_response::<FirebaseAdminLookupResponse>(response).await {
                Ok(users) => {
                    println!("Successfully retrieved auth data");
                    users
                }
                Err((status, error)) => {
                    println!(
                        "Error handling Firebase Admin response: {} - {:?}",
                        status, error
                    );
                    return (status, Json(error)).into_response();
                }
            }
        }
        Err(e) => {
            println!("Error connecting to Firebase Admin: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Error connecting to Firebase".to_string(),
                )),
            )
                .into_response();
        }
    };

    let merged_users: Vec<UserMerged> = user_data_auth.merge(user_data_db);
    (
        StatusCode::OK,
        Json(ResponseAPI::<Vec<UserMerged>>::success(
            "Users retrieved successfully".to_string(),
            merged_users,
        )),
    )
        .into_response()
}

// Obtener datos del usuario según su sesión de Firebase DB
#[instrument(
    skip(state, id_token),
    fields(
        user_id = %user_claims.sub,
        operation = "get_user_data_db"
    )
)]
pub async fn get_user_data_db(
    user_claims: &UserAuthentication,
    id_token: &str,
    state: &Arc<AppState>,
) -> Option<UserDB> {
    // URL de Firebase Realtime Database para obtener los datos del usuario
    let url_firebase_db: String = format!(
        "{}/user_profiles/{}.json?auth={}",
        state.firebase_options.firebase_database_url, user_claims.sub, id_token
    );

    // Realizamos la petición a Firebase Realtime Database
    match state
        .firebase_options
        .firebase_client
        .get(url_firebase_db)
        .send()
        .await
    {
        Ok(response) => {
            println!("Firebase DB response status: {}", response.status());
            match handle_firebase_response::<UserDB>(response).await {
                Ok(user) => Some(user),
                Err((_, _)) => None,
            }
        }
        Err(_) => None,
    }
}

// Refrescar el token
#[debug_handler]
#[instrument(skip(state, refresh_token), fields(operation = "refresh_token"))]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(refresh_token): Json<RefreshToken>,
) -> impl IntoResponse {
    // URL de Firebase para refrescar el token
    let url_firebase_auth_refresh_token: String = format!(
        "https://securetoken.googleapis.com/v1/token?key={}",
        state.firebase_options.firebase_api_key
    );

    // Actualizamos el neuvo token, no hace falta devolber nada
    match state
        .firebase_options
        .firebase_client
        .post(&url_firebase_auth_refresh_token)
        .json(&refresh_token)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseAuthResponse>(response).await {
            Ok(auth_response) => (
                StatusCode::OK,
                Json(ResponseAPI::<String>::success(
                    "Token refreshed successfully".to_string(),
                    auth_response.id_token,
                )),
            )
                .into_response(),
            Err(_) => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseAPI::<()>::error(
                    "Invalid refresh token".to_string(),
                )),
            )
                .into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Error refreshing token".to_string(),
            )),
        )
            .into_response(),
    }
}

// Eliminar el usuario actualmente autentificado
#[debug_handler]
#[instrument(
    skip(state, user_claims, id_token),
    fields(
        user_id = %user_claims.sub,
        operation = "delete_me"
    )
)]
pub async fn delete_me(
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // URL para eliminar el usuario especificado en Firebase Authentication
    let url_firebase_auth: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:delete?key={}",
        state.firebase_options.firebase_api_key
    );

    // Obtenemos el usuario de las claims y lo borramos
    // Si falló la eliminación de Auth, retornar error inmediatamente
    let _: Response<Body> = match state
        .firebase_options
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
            Json(ResponseAPI::<()>::error(
                "Error connecting to Firebase".to_string(),
            )),
        )
            .into_response(),
    };

    // URL de para eliminar usuario en la DB
    let url_firebase_db: String = format!(
        "{}/user_profiles/{}.json?auth={}",
        state.firebase_options.firebase_database_url, user_claims.sub, id_token
    );

    // Eliminamos el usuario de Firebase Realtime Database
    match state
        .firebase_options
        .firebase_client
        .delete(url_firebase_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => (StatusCode::NO_CONTENT).into_response(),
        Ok(_) => (
            StatusCode::BAD_REQUEST,
            Json(ResponseAPI::<()>::error("Invalid credential".to_string())),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Error connecting to Firebase".to_string(),
            )),
        )
            .into_response(),
    }
}
