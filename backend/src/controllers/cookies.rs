use {
    crate::{
        middleware::auth::verify_firebase_token,
        models::{
            cookies::{SessionData, SessionRequest},
            firebase::UserAuthentication,
            response::ResponseAPI,
        },
        state::AppState,
    },
    axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse},
    jsonwebtoken::TokenData,
    std::sync::Arc,
    time::Duration,
    tower_cookies::{Cookie, Cookies, cookie::SameSite},
    tracing::{debug, info, instrument, warn},
};

// Guardar la session en una cookie
#[debug_handler]
pub async fn add_session(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
    Json(request): Json<SessionRequest>,
) -> impl IntoResponse {
    info!("Creating user session");

    // Validar token con Firebase Admin (que ya tienes funcionando)
    let decoded: TokenData<UserAuthentication> = match verify_firebase_token(
        &request.token,
        &state.firebase.firebase_keys,
        &state.firebase.firebase_project_id,
    ) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ResponseAPI::<()>::error("Invalid token".to_string())),
            )
                .into_response();
        }
    };

    let session_data: SessionData = SessionData {
        token: request.token,
        local_id: decoded.claims.sub,
        email: decoded.claims.email,
        name: decoded.claims.name,
        exp: decoded.claims.exp as u64,
        picture: decoded.claims.picture,
        email_verified: decoded.claims.email_verified.unwrap_or(false),
        provider: Some(match &decoded.claims.firebase {
            Some(firebase_info) => firebase_info.sign_in_provider.clone(),
            None => "unknown".to_string(),
        }),
    };

    // Serializar datos
    let session_json: String = match serde_json::to_string(&session_data) {
        Ok(json) => {
            debug!(json_length = json.len(), "Session data serialized");
            json
        }
        Err(e) => {
            warn!("Failed to serialize session data: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Failed to create session".to_string(),
                )),
            )
                .into_response();
        }
    };

    // Use tower_cookies::Cookie with the correct API
    let cookie: Cookie = tower_cookies::Cookie::build(("session", session_json))
        .path("/")
        .http_only(true)
        .secure(cfg!(not(debug_assertions))) // HTTP en desarrollo, HTTPS en producción
        .same_site(if cfg!(debug_assertions) {
            SameSite::Lax // Desarrollo: más permisivo
        } else {
            SameSite::Strict // Producción: máxima seguridad
        })
        .max_age(Duration::days(7)) // Use the time crate Duration
        .build();

    cookies.add(cookie);

    info!("Session created successfully");
    (
        StatusCode::CREATED,
        Json(ResponseAPI::<()>::success_no_data()),
    )
        .into_response()
}

// Devolber la session de la cookie
#[debug_handler]
pub async fn get_session(cookies: Cookies) -> impl IntoResponse {
    match cookies.get("session") {
        Some(cookie) => match serde_json::from_str::<SessionData>(cookie.value()) {
            Ok(session_data) => (
                StatusCode::OK,
                Json(ResponseAPI::success(
                    "Session retrieved successfully".to_string(),
                    session_data,
                ))
                .into_response(),
            ),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseAPI::<()>::error("Invalid session data".to_string()))
                        .into_response(),
                );
            }
        },
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("No session found".to_string())).into_response(),
            );
        }
    }
}

// Eliminar la session (logout)
#[debug_handler]
#[instrument(skip(cookies))]
pub async fn delete_session(cookies: Cookies) -> impl IntoResponse {
    info!("Starting session deletion process");

    match cookies.get("session") {
        Some(cookie) => {
            // Intentar deserializar para obtener info del usuario antes de eliminar
            if let Ok(session_data) = serde_json::from_str::<SessionData>(cookie.value()) {
                info!(
                    user_id = %session_data.local_id,
                    email = %session_data.email.as_deref().unwrap_or(""),
                    "Logging out user"
                );
            }

            // Crear cookie con expiración inmediata para eliminarla
            let expired_cookie = Cookie::build(("session", ""))
                .path("/")
                .http_only(true)
                .secure(cfg!(not(debug_assertions))) // ← Usar mismo patrón
                .same_site(if cfg!(debug_assertions) {
                    SameSite::Lax
                } else {
                    SameSite::Strict
                })
                .max_age(Duration::seconds(0))
                .build();

            cookies.add(expired_cookie);

            info!("Session cookie deleted successfully");

            (StatusCode::OK, Json(ResponseAPI::<()>::success_no_data())).into_response()
        }
        None => {
            warn!("Attempted to delete non-existent session");
            (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error(
                    "No session found to delete".to_string(),
                )),
            )
                .into_response()
        }
    }
}
