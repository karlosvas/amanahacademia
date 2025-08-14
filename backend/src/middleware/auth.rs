use {
    crate::{
        models::{error::AuthError, user::UserAuthentication},
        state::AppState,
    },
    axum::{
        extract::{Request, State},
        http::StatusCode,
        middleware::Next,
        response::Response,
    },
    jsonwebtoken::{Algorithm, DecodingKey, Header, TokenData, Validation, decode},
    std::sync::Arc,
    tracing::error,
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

pub async fn firebase_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extraer y validar el token
    let token: &str = extract_bearer_token(&request)?;

    // Verificar el token y obtener los claims del usuario
    let user_claims: TokenData<UserAuthentication> =
        verify_firebase_token(token, &state.firebase.firebase_keys).map_err(|err| {
            error!("Token verification failed: {}", err);
            StatusCode::from(err)
        })?;

    // Agregar los claims del usuario a las extensiones del request
    // para que puedan ser utilizados en los handlers
    request.extensions_mut().insert(user_claims.claims);

    Ok(next.run(request).await)
}

// Manejamos errores de extraer el token
fn extract_bearer_token(request: &Request) -> Result<&str, AuthError> {
    // Obtenemos el token del header si no esta lanzamos un error de no autorizado
    let auth_header = request
        .headers()
        .get("authorization")
        .ok_or(AuthError::MissingHeader)?;

    // Verificamos que el header tenga el formato correcto
    let auth_str = auth_header
        .to_str()
        .map_err(|_| AuthError::InvalidHeaderFormat)?;

    // Devolbemos el token
    auth_str
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidHeaderFormat)
}

// Verificamos que el token sea un de Firebase válido
fn verify_firebase_token(
    token: &str,
    firebase_keys: &serde_json::Value,
) -> Result<TokenData<UserAuthentication>, AuthError> {
    // Extrae el kid del token
    let header: Header = jsonwebtoken::decode_header(token)
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;
    let kid: String = header.kid.ok_or(AuthError::MissingKid)?;

    // Obtén la clave pública correspondiente
    let public_key_pem = firebase_keys
        .get(&kid)
        .and_then(|key| key.as_str())
        .ok_or(AuthError::NoMatchingKey)?;

    // Configura la validación del token
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["your-project-id"]); // Reemplaza con tu project ID
    validation.set_issuer(&[&format!("https://securetoken.google.com/your-project-id")]); // Reemplaza con tu project ID

    // Verifica el token
    let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;

    let token_data = decode::<UserAuthentication>(token, &decoding_key, &validation)
        .map_err(|e| AuthError::TokenVerification(e.to_string()))?;

    Ok(token_data)
}
