use {
    serde::{Deserialize, Serialize},
    std::collections::{HashMap, HashSet},
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
    pub permissions: Option<HashSet<String>>,
    pub subscription_tier: Option<String>,
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
#[derive(Debug, Deserialize, Serialize)]
pub struct FirebaseAuthResponse {
    #[serde(rename = "localId")]
    pub local_id: String, // ID del usuario en Firebase
    pub email: String, // Email del usuario
    #[serde(rename = "idToken")]
    pub id_token: String, // Token de identificación
    #[serde(rename = "refreshToken")]
    pub refresh_token: String, // Token de actualización
    #[serde(rename = "expiresIn")]
    pub expires_in: String, // "3600" (segundos)
    #[serde(rename = "registered", default)]
    pub registered: Option<bool>, // Si es usuario nuevo o existente
}

// Info de firebase
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseInfo {
    pub identities: HashMap<String, Vec<String>>,
    pub sign_in_provider: String,
}

// Usuario de la base de datos
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserDB {
    pub email: String,
    pub role: Option<String>,
    pub subscription_tier: Option<String>,
    pub permissions: Option<HashSet<String>>,
}

// Usuario de la FB auth
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserAuth {
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    pub email: String,
    pub password: String,
    pub return_secure_token: bool,
}

pub type UserDBResponse = HashMap<String, UserDB>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FirebaseAccountsLookupResponse {
    #[serde(default)]
    pub users: Vec<FirebaseUserInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseUserInfo {
    #[serde(rename = "localId")]
    pub local_id: String, // UID del usuario

    pub email: Option<String>,
    #[serde(rename = "emailVerified")]
    pub email_verified: Option<bool>,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,

    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,

    pub disabled: Option<bool>,

    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Option<Vec<ProviderUserInfo>>,

    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,

    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: Option<f64>,

    #[serde(rename = "validSince")]
    pub valid_since: Option<String>,

    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,

    #[serde(rename = "customAuth")]
    pub custom_auth: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderUserInfo {
    #[serde(rename = "providerId")]
    pub provider_id: String, // "password", "google.com", etc.

    #[serde(rename = "federatedId")]
    pub federated_id: Option<String>,

    pub email: Option<String>,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,

    #[serde(rename = "rawId")]
    pub raw_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RefreshToken {
    pub grant_type: String,
    pub refresh_token: String,
}
