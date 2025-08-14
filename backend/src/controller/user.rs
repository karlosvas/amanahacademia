use {
    crate::{
        models::user::{
            CreateUserFirebaseAuthentication, FirebaseAuthResponse, UserDB, UserRequest,
        },
        state::AppState,
    },
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    reqwest::Client,
    serde_json::json,
    std::sync::Arc,
};

// Creación del usuario conn firebase
#[axum::debug_handler()]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
    // Tu middleware ya se encarga de la autenticación
    let client: Client = Client::new();
    let url_firebase_auth: String = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
        state.firebase.firebase_api_key
    );

    // Creamos el usuario que se va a crear en Firebase Authentication
    let new_user_authentication: CreateUserFirebaseAuthentication =
        CreateUserFirebaseAuthentication {
            email: user.email.clone(),
            password: user.password,
            return_secure_token: true,
        };
    // Crear usuario en Firebase Authentication
    let auth_response: FirebaseAuthResponse = match client
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
                        "Error al procesar respuesta de Firebase Auth",
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
                "Error al crear usuario en Firebase Auth",
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error de conexión con Firebase Auth",
            )
                .into_response();
        }
    };

    // Crear usuario en la base de datos
    let user_db: UserDB = UserDB {
        uid: auth_response.local_id.clone(),
        email: auth_response.email.clone(),
        role: user.role,
        subscription_tier: user.subscription_tier,
        permissions: user.permissions,
    };
    let url_firebase_db = format!(
        "https://{}-default-rtdb.europe-west1.firebasedatabase.app/user_profiles/{}.json?auth={}",
        state.firebase.firebase_project_id, auth_response.local_id, auth_response.id_token
    );
    // Guardar perfil en Realtime Database
    match client.put(&url_firebase_db).json(&user_db).send().await {
        Ok(response) if response.status().is_success() => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "user_id": auth_response.local_id,
                "email": auth_response.email,
                "message": "Usuario creado exitosamente"
            })),
        )
            .into_response(),
        Ok(_) => {
            // Usuario creado en Auth pero falló en DB
            (
                StatusCode::PARTIAL_CONTENT,
                Json(json!({
                    "success": false,
                    "user_id": auth_response.local_id,
                    "email": auth_response.email,
                    "warning": "Usuario creado en Firebase Auth pero error al guardar perfil en la base de datos"
                })),
            )
                .into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al guardar perfil").into_response(),
    }
}

// pub async fn obtain_user(
//     State(state): State<Arc<AppState>>,
//     Extension(uid): Extension<String>,
// ) -> impl IntoResponse {
//     // // Tu middleware ya se encarga de la autenticación
//     // let client: Client = Client::new();
//     // let url: String = format!(
//     //     "https://{}.firebaseio.com/users/{}.json?auth={}",
//     //     state.firebase.firebase_project_id, uid, state.firebase.firebase_api_key
//     // );

//     // match client.get(&url).send().await {
//     //     Ok(response) if response.status().is_success() => {
//     //         let user: UserAuthentication = response.json().await.unwrap();
//     //         (StatusCode::OK, Json(user))
//     //     }
//     //     Ok(_) => (StatusCode::NOT_FOUND, "Usuario no encontrado"),
//     //     Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error de conexión"),
//     // }
// }

// Actualización de usuario - REQUIERE autenticación
// pub async fn update_user(
//     request: Request,
//     Path(user_id): Path<String>, // ID del usuario a actualizar
//     Json(user_data): Json<User>, // Nuevos datos del usuario
// ) -> impl IntoResponse {
//     // Obtener el usuario autenticado del middleware
//     let current_user = match request.extensions().get::<User>() {
//         Some(user) => user,
//         None => return (StatusCode::UNAUTHORIZED, "No autenticado").into_response(),
//     };
// }
