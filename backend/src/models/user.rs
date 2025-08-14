use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

// Estructura de los datos que se pasan desde el frontend para los enpoints de users
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    // Datos obligatorios requeridos por firebase auth
    pub email: String,
    pub password: String,

    // Datos opcionales que el cliente puede enviar
    pub name: Option<String>,
    pub phone_number: Option<String>,

    // Datos especificos para la DB
    pub role: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub subscription_tier: Option<String>,
}

// Estructura para crear usuario en Firebase Auth API
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserFirebaseAuthentication {
    // Datos obligatorios requeridos por firebase auth
    pub email: String,
    pub password: String,
    #[serde(rename = "returnSecureToken")]
    pub return_secure_token: bool,
}

// Para claims del JWT (usuario autenticado)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuthentication {
    // Claims estándar JWT
    pub sub: String, // Subject (user ID)
    pub iss: String, // Issuer
    pub aud: String, // Audience
    pub iat: i64,    // Issued at
    pub exp: i64,    // Expiration

    // Claims específicos de Firebase
    pub email: Option<String>,          // Email del usuario
    pub email_verified: Option<bool>,   // Si el email está verificado
    pub name: Option<String>,           // Nombre completo
    pub picture: Option<String>,        // URL de la foto de perfil
    pub auth_time: i64,                 // Timestamp de autenticación
    pub user_id: String,                // ID del usuario en Firebase
    pub firebase: Option<FirebaseInfo>, // Información específica de Firebase

    // Campos adicionales que Firebase puede proporcionar
    pub phone_number: Option<String>, // Número de teléfono
    pub provider_id: Option<String>,  // ID del proveedor de autenticación
}

// Para respuesta de Firebase Auth
#[derive(Debug, Deserialize)]
pub struct FirebaseAuthResponse {
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String, // "3600" (segundos)
    pub registered: Option<bool>, // Si es usuario nuevo o existente
}

// Info de firebase
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseInfo {
    pub identities: HashMap<String, Vec<String>>,
    pub sign_in_provider: String,
}

// Usuario de la base de datos
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDB {
    pub uid: String,
    pub email: String,
    pub role: Option<String>,
    pub subscription_tier: Option<String>,
    pub permissions: Option<Vec<String>>,
}
