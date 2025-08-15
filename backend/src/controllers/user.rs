use {
    crate::{
        models::user::{
            FirebaseAuthResponse, UserAuthentication, UserDB, UserDBResponse, UserRequest,
        },
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
    serde_json::{Value, json},
    std::{collections::HashMap, sync::Arc},
};

// Creación del usuario conn firebase
#[debug_handler()]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
    // Obtenemos la URL de registro de usuario con Firebase
    let url_firebase_auth: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        state.firebase.firebase_api_key
    );

    // Creamos el usuario que se va a crear en Firebase Authentication
    let new_user_authentication: Value = json!({
        "email": user.email.clone(),
        "password": user.password,
        "return_secure_token": true,
    });

    // POST:: Crear usuario en Firebase Authentication
    let auth_response: FirebaseAuthResponse = match state
        .client
        .post(&url_firebase_auth)
        .json(&new_user_authentication)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<FirebaseAuthResponse>().await {
                Ok(user) => user,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error processing Firebase Auth response",
                    )
                        .into_response();
                }
            }
        }
        Ok(response) => {
            let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let error_text: String = response.text().await.unwrap_or_default();
            eprintln!("Firebase Auth Error: {} - {}", status, error_text);
            return (
                StatusCode::BAD_REQUEST,
                "Error creating user in Firebase Auth",
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error connecting to Firebase Auth",
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

    // Post:: crear usuario
    match state
        .client
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
#[debug_handler()]
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
    let login_payload: Value = json!({
        "email": login.email,
        "password": login.password,
        "returnSecureToken": true
    });

    // Enviar la solicitud a Firebase Auth
    match state
        .client
        .post(&url_login_firebase)
        .json(&login_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<FirebaseAuthResponse>().await {
                Ok(auth_response) => (
                    StatusCode::OK,
                    Json(json!({ "success": true, "body": { "token": auth_response.id_token, "message": "Login successful" } })),
                )
                    .into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Error processing Firebase response" })),
                )
                    .into_response(),
            }
        }
        Ok(response) => {
            let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let error_text: String = response.text().await.unwrap_or_default();
            eprintln!("Firebase Auth Error: {} - {}", status, error_text);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid credential" })),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    }
}

// Actualización de usuario
#[debug_handler()]
pub async fn update_user(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
    Json(user_request): Json<UserRequest>,
) -> impl IntoResponse {
    println!("user claims: {:?}", user_claims);
    println!("id token: {:?}", id_token);

    // Comprobación de campos obligatorios
    if user_request.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Email is required" })),
        )
            .into_response();
    }

    // Obtener los datos del usuario actual
    let actual_user_db: UserDB = match get_user_data(&user_claims, &id_token, &state).await {
        Some(user_db) => user_db,
        None => {
            eprintln!("Error getting user data");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Error getting user data" })),
            )
                .into_response();
        }
    };

    println!("Actual user DB: {:?}", actual_user_db);

    // URL para la actualización de usuario en Firebase
    let url_firebase_auth_update: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:update?key={}",
        state.firebase.firebase_api_key
    );

    // Cuerpo de la solicitud para la actualización de usuario
    let user_payload: Value = json!({
        "email": user_request.email, // El email es obligatorio darlo en la request
        "password": user_request.password, // La contraseña es obligatoria darle en la request
        "returnSecureToken": true
    });

    // Enviar la solicitud a Firebase Auth
    let _fb_auth_updated: Response<Body> = match state
        .client
        .put(url_firebase_auth_update)
        .json(&user_payload)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<FirebaseAuthResponse>().await {
                Ok(_) => (
                    StatusCode::OK,
                    Json(json!({ "success": true, "message": "Login successful" })),
                )
                    .into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Error processing Firebase response" })),
                )
                    .into_response(),
            }
        }
        Ok(response) => {
            let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let error_text: String = response.text().await.unwrap_or_default();
            eprintln!("Firebase Auth Error: {} - {}", status, error_text);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid credential" })),
            )
                .into_response()
        }
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
        .client
        .put(url_firebase_db)
        .json(&user_db)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<FirebaseAuthResponse>().await {
                Ok(_) => (
                    StatusCode::OK,
                    Json(json!({ "success": true, "message": "Login successful" })),
                )
                    .into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Error processing Firebase response" })),
                )
                    .into_response(),
            }
        }
        Ok(response) => {
            let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let error_text: String = response.text().await.unwrap_or_default();
            eprintln!("Firebase Auth Error: {} - {}", status, error_text);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid credential" })),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    }
}

// Eliminar el usuario actualmente autentificado
#[debug_handler()]
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
    let auth_result = state
        .client
        .post(url_firebase_auth) // Fb utiliza POST para eliminar usuarios
        .json(&json!({ "idToken": id_token }))
        .send()
        .await;

    // Si falló la eliminación de Auth, retornar error inmediatamente
    let response = match auth_result {
        Ok(response) if response.status().is_success() => (StatusCode::NO_CONTENT).into_response(),
        Ok(response) => {
            // Si se puedo hacer la peticion pero no se obtuvo una respuesta exitosa, obtenemos el estado
            let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            // Obtenemos el mensaje de error
            let error_text: String = response.text().await.unwrap_or_default();
            eprintln!("Firebase Auth Error: {} - {}", status, error_text);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid credential" })),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    };

    // Si falló la eliminación de Auth, retornar error inmediatamente
    if !response.status().is_success() {
        return (response.status(), response.into_body()).into_response();
    }

    // URL de para eliminar usuario en la DB
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/user_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, user_claims.sub, id_token
    );

    // Eliminamos el usuario de Firebase Realtime Database
    match state.client.delete(url_firebase_db).send().await {
        Ok(response) if response.status().is_success() => (StatusCode::NO_CONTENT).into_response(),
        Ok(response) => {
            let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let error_text: String = response.text().await.unwrap_or_default();
            eprintln!("Firebase Auth Error: {} - {}", status, error_text);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid credential" })),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error connecting to Firebase" })),
        )
            .into_response(),
    }
}

// Obtener todos los usuarios
#[debug_handler()]
pub async fn get_all_users(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos de la base de datos el usuario actual
    let actual_user_db: UserDB = match get_user_data(&user_claims, &id_token, &state).await {
        Some(user_db) => user_db,
        None => {
            eprintln!("Error getting user data");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Error getting user dataa" })),
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
    let user_data_db: UserDBResponse = match state.client.get(&url_firebase_db).send().await {
        Ok(response) if response.status().is_success() => match response.text().await {
            Ok(response_text) => {
                if response_text.trim().is_empty() || response_text.trim() == "null" {
                    HashMap::new()
                } else {
                    println!("Response text: {}", response_text);
                    match serde_json::from_str::<UserDBResponse>(&response_text) {
                        Ok(value) => value,
                        Err(e) => {
                            eprintln!("Error parsing JSON: {}", e);
                            HashMap::new()
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading response text: {}", e);
                HashMap::new()
            }
        },
        Ok(response) => {
            eprintln!(
                "Firebase DB Error - Status: {}, Body: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Error retrieving users from database" })),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Connection error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Error connecting to Firebase" })),
            )
                .into_response();
        }
    };

    // URL de los datos de Firebase Admin
    // let url_firebase_admin: String = format!(
    //     // Obtenemos la información del usuario de Firebase Realtime Database
    //     "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
    //     state.firebase.firebase_api_key
    // );

    // // Realizamos la petición a Firebase Admin para obtener la información del usuario
    // let user_data_auth: FirebaseAccountsLookupResponse =
    //     match state.client.post(&url_firebase_admin).send().await {
    //         Ok(response) if response.status().is_success() => {
    //             match response.json::<FirebaseAccountsLookupResponse>().await {
    //                 Ok(users) => users,
    //                 Err(_) => {
    //                     eprintln!("Error parsing database users data");
    //                     return (
    //                         StatusCode::INTERNAL_SERVER_ERROR,
    //                         Json(json!({ "error": "Error parsing database users data" })),
    //                     )
    //                         .into_response();
    //                 }
    //             }
    //         }
    //         Ok(response) => {
    //             let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
    //                 .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    //             let error_text: String = response.text().await.unwrap_or_default();
    //             eprintln!("Firebase Admin Error: {} - {}", status, error_text);
    //             FirebaseAccountsLookupResponse::default()
    //         }
    //         Err(_) => {
    //             eprintln!("Error connecting to Firebase");
    //             FirebaseAccountsLookupResponse::default()
    //         }
    //     };

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "body": { "users": {
                // "firebase": user_data_auth,
                "database": user_data_db
            }, "message": "Users retrieved successfully" }
        })),
    )
        .into_response()
}

// Obtener datos del usuario según su sesión
async fn get_user_data(
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
    match state.client.get(url_firebase_db).send().await {
        Ok(response) if response.status().is_success() => match response.text().await {
            Ok(response_text) => match serde_json::from_str::<UserDB>(&response_text) {
                Ok(user) => Some(user),
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    eprintln!("JSON recibido: {}", response_text);
                    None
                }
            },
            Err(e) => {
                eprintln!("Error leyendo respuesta: {}", e);
                None
            }
        },
        Ok(response) => {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            eprintln!(
                "Firebase DB Error - Status: {}, Body: {}",
                status, error_text
            );
            None
        }
        Err(e) => {
            eprintln!("Connection error: {}", e);
            None
        }
    }
}
