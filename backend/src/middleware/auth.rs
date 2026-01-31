use {
    crate::models::{
        error::AuthError,
        firebase::UserAuthentication,
        metrics::{ClaimsGA, ServiceAccount, TokenResponse},
        state::AppState,
    },
    axum::{
        extract::{Request, State},
        http::{HeaderValue, StatusCode},
        middleware::Next,
        response::Response,
    },
    jsonwebtoken::{
        Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    },
    reqwest::Client as HttpClient,
    serde_json::Value,
    std::{
        sync::Arc,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tracing::{error, info, instrument, warn},
};

#[instrument(skip(state, request, next))]
pub async fn firebase_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract and validate the token
    let token: String = {
        let token_str: &str = extract_bearer_token(&request)?;
        token_str.to_string() // Convertimos a `String` para evitar problemas de lifetime
    };

    // Obtener claves (con refresh si necesario)
    let firebase_keys = get_or_refresh_keys(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Verificar el token y obtener los claims del usuario
    let user_claims: TokenData<UserAuthentication> = verify_firebase_token(
        &token,
        &firebase_keys,
        &state.firebase_options.firebase_project_id,
    )
    .map_err(|err| {
        warn!("Auth failed: {}", err); // ← Solo warn, no error
        StatusCode::from(err)
    })?;

    // Loguear el user_id para trazabilidad
    tracing::Span::current().record("user_id", &user_claims.claims.user_id);

    // Agregar los claims del usuario a las extensiones del request
    // para que puedan ser utilizados en los handlers
    request.extensions_mut().insert(user_claims.claims);

    // Agregar el token original a las extensiones del request
    request.extensions_mut().insert(token.to_string());

    Ok(next.run(request).await)
}

// Manejamos errores de extraer el token
#[instrument]
fn extract_bearer_token(request: &Request) -> Result<&str, AuthError> {
    // Obtenemos el token del header si no esta lanzamos un error de no autorizado
    let auth_header: &HeaderValue = request
        .headers()
        .get("authorization")
        .ok_or(AuthError::MissingHeader)?;

    // Verificamos que el header tenga el formato correcto
    let auth_str: &str = auth_header
        .to_str()
        .map_err(|_| AuthError::InvalidHeaderFormat)?;

    // Devolbemos el token
    auth_str
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidHeaderFormat)
}

// Verificamos que el token sea un de Firebase válido
#[instrument(skip(token, firebase_keys))]
pub fn verify_firebase_token(
    token: &str,
    firebase_keys: &serde_json::Value,
    project_id: &str,
) -> Result<TokenData<UserAuthentication>, AuthError> {
    // Extrae el kid del token
    let header: Header = jsonwebtoken::decode_header(token)
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;
    let kid: String = header.kid.ok_or(AuthError::MissingKid)?;

    // Loguea el kid para trazabilidad
    tracing::Span::current().record("kid", &kid.as_str());

    // Obtén la clave pública correspondiente
    let public_key_pem: &str = firebase_keys
        .get(&kid)
        .and_then(|key| key.as_str())
        .ok_or(AuthError::NoMatchingKey)?;

    // Configura la validación del token
    let mut validation: Validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["amanahacademia"]); // Reemplaza con tu project ID
    validation.set_issuer(&[&format!("https://securetoken.google.com/amanahacademia")]); // Reemplaza con tu project ID

    // Verifica el token
    let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;

    let token_data = decode::<UserAuthentication>(token, &decoding_key, &validation)
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;

    Ok(token_data)
}

/// Función que maneja el refresh
async fn get_or_refresh_keys(
    state: &Arc<AppState>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Leer caché
    {
        let cache = state.firebase_options.firebase_keys.read().await;
        if !cache.is_expired() {
            return Ok(cache.keys.clone());
        }
    }

    // Caché expirado, refrescar
    warn!("Firebase keys expired, refreshing...");

    let new_keys = fetch_firebase_keys_internal(&state.firebase_options.firebase_client).await?;

    // Actualizar caché
    {
        let mut cache = state.firebase_options.firebase_keys.write().await;
        cache.keys = new_keys.clone();
        cache.fetched_at = SystemTime::now();
    }

    info!("Firebase keys refreshed successfully");
    Ok(new_keys)
}

/// Función interna para obtener las claves públicas de Firebase
async fn fetch_firebase_keys_internal(
    client: &HttpClient,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let response = client
        .get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err("Firebase keys endpoint error".into());
    }

    let keys: Value = response.json().await?;

    if !keys.is_object() || keys.as_object().map_or(true, |m| m.is_empty()) {
        return Err("Empty Firebase keys".into());
    }

    Ok(keys)
}

/// Midelware para obtener el token de Goolge Analytics sin verificar usuario
pub async fn public_ga_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Solo obtener el token de Google Analytics (sin verificar usuario)
    let ga_token: String = get_ga_token(&state).await.map_err(|e| {
        error!("Failed to get GA token: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Agregar el token de GA a las extensiones del request
    request
        .extensions_mut()
        .insert(crate::models::metrics::GAToken(ga_token));

    Ok(next.run(request).await)
}

/// Función auxiliar para obtener el token de Google Analytics
async fn get_ga_token(state: &Arc<AppState>) -> Result<String, Box<dyn std::error::Error>> {
    let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let service_account: &ServiceAccount = &state.ga_options.service_account;

    let claims: ClaimsGA = ClaimsGA {
        iss: service_account.client_email.clone(),
        scope: "https://www.googleapis.com/auth/analytics.readonly".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    let encoding_key: EncodingKey =
        EncodingKey::from_rsa_pem(service_account.private_key.as_bytes())?;
    let jwt: String = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)?;

    let response: TokenResponse = state
        .ga_options
        .client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await?
        .json()
        .await?;

    Ok(response.access_token)
}

#[cfg(test)]
#[path = "../test/middleware/auth.rs"]
mod extended_tests;
