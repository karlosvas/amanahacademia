use {
    crate::{
        models::{error::AuthError, firebase::UserAuthentication},
        state::AppState,
    },
    axum::{
        extract::{Request, State},
        http::{HeaderValue, StatusCode},
        middleware::Next,
        response::Response,
    },
    jsonwebtoken::{Algorithm, DecodingKey, Header, TokenData, Validation, decode},
    std::sync::Arc,
    tracing::{debug, info, instrument},
};

impl From<AuthError> for StatusCode {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::MissingHeader | AuthError::InvalidHeaderFormat => StatusCode::UNAUTHORIZED,
            AuthError::TokenVerification(_)
            | AuthError::MissingKid
            | AuthError::NoMatchingKey
            | AuthError::InvalidKeyFormat => StatusCode::FORBIDDEN,
        }
    }
}

#[instrument(skip(state, request, next))]
pub async fn firebase_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    debug!("Firebase auth middleware started");

    // Extract and validate the token
    let token: String = {
        let token_str: &str = extract_bearer_token(&request)?;
        token_str.to_string() // Convertimos a `String` para evitar problemas de lifetime
    };

    // Verificar el token y obtener los claims del usuario
    let user_claims: TokenData<UserAuthentication> = match verify_firebase_token(
        &token,
        &state.firebase_options.firebase_keys,
        &state.firebase_options.firebase_project_id,
    ) {
        Ok(claims) => claims,
        Err(err) => return Err(StatusCode::from(err)),
    };

    // Agregar los claims del usuario a las extensiones del request
    // para que puedan ser utilizados en los handlers
    request.extensions_mut().insert(user_claims.claims);

    // Agregar el token original a las extensiones del request
    request.extensions_mut().insert(token.to_string());

    debug!("Auth middleware completed successfully");
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

    debug!(auth_header_length = %auth_str.len(), "Authorization header found");
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
    debug!("Starting Firebase token verification");

    // Extrae el kid del token
    let header: Header = jsonwebtoken::decode_header(token)
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;
    let kid: String = header.kid.ok_or(AuthError::MissingKid)?;

    debug!(kid = %kid, "Token kid extracted");

    // Obtén la clave pública correspondiente
    let public_key_pem: &str = firebase_keys
        .get(&kid)
        .and_then(|key| key.as_str())
        .ok_or(AuthError::NoMatchingKey)?;

    debug!("Public key found for kid");
    // Configura la validación del token
    let mut validation: Validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["amanahacademia"]); // Reemplaza con tu project ID
    validation.set_issuer(&[&format!("https://securetoken.google.com/amanahacademia")]); // Reemplaza con tu project ID

    debug!(
        audience = %project_id,
        issuer = %format!("https://securetoken.google.com/{}", project_id),
        "Token validation configured"
    );

    // Verifica el token
    let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;

    let token_data = decode::<UserAuthentication>(token, &decoding_key, &validation)
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;

    info!("Token verification completed successfully");
    Ok(token_data)
}
