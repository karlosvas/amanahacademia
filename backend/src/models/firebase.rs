use {
    crate::models::user::UserDB,
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

/// Token usado para refrescar la sesión cuando el idToken de Firebase expira
#[derive(Deserialize, Serialize)]
pub struct RefreshToken {
    pub grant_type: String,
    pub refresh_token: String,
}

/// Payload para autenticación de usuarios con email/password en Firebase
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserAuth {
    /// Token JWT previo (usado para renovación de sesión)
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    pub email: String,
    pub password: String,
    /// Indica si Firebase debe devolver el refresh token
    pub return_secure_token: bool,
    pub display_name: Option<String>,
}

/// Claims (declaraciones) del JWT de Firebase después de autenticar un usuario
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuthentication {
    // === Claims estándar JWT (RFC 7519) ===
    /// Subject: identificador único del usuario
    pub sub: String,
    /// Issuer: quién emitió el token (Firebase)
    pub iss: String,
    /// Audience: para quién está destinado el token (tu proyecto Firebase)
    pub aud: String,
    /// Issued at: timestamp de emisión del token
    pub iat: i64,
    /// Expiration: timestamp de expiración del token
    pub exp: i64,

    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    /// Timestamp de cuándo el usuario se autenticó originalmente
    pub auth_time: i64,
    pub user_id: String,
    pub firebase: Option<FirebaseInfo>,
    pub phone_number: Option<String>,
    /// Proveedor de autenticación: "password", "google.com", etc.
    pub provider_id: Option<String>,
}

/// Respuesta de Firebase Auth tras login/signup exitoso
#[derive(Debug, Deserialize, Serialize)]
pub struct FirebaseAuthResponse {
    /// ID único del usuario en Firebase
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    /// JWT para autenticar requests subsecuentes
    #[serde(rename = "idToken")]
    pub id_token: String,
    /// Token para obtener un nuevo idToken cuando expire
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    /// Tiempo de expiración en segundos (ej: "3600" = 1 hora)
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    /// true = login de usuario existente, false = nuevo registro
    #[serde(rename = "registered", default)]
    pub registered: Option<bool>,
}

/// Información completa de un usuario obtenida desde Firebase Admin API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseUserInfo {
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: Option<String>,
    #[serde(rename = "emailVerified")]
    pub email_verified: Option<bool>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    /// Si true, el usuario está deshabilitado y no puede autenticarse
    pub disabled: Option<bool>,
    /// Lista de proveedores OAuth vinculados (Google, Facebook, etc.)
    #[serde(rename = "providerUserInfo")]
    pub provider_user_info: Option<Vec<ProviderUserInfo>>,
    /// Hash de la contraseña (solo disponible con permisos admin)
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    /// Timestamp de última actualización de contraseña (milisegundos)
    #[serde(rename = "passwordUpdatedAt")]
    pub password_updated_at: Option<f64>,
    /// Timestamp desde el cual los tokens son válidos (útil para invalidar sesiones)
    #[serde(rename = "validSince")]
    pub valid_since: Option<String>,
    /// Timestamp del último login (formato string de milisegundos)
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<String>,
    /// Timestamp de creación del usuario (formato string de milisegundos)
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "customAuth")]
    pub custom_auth: Option<bool>,
}

/// Información del proveedor de autenticación (Google, Password, etc.)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderUserInfo {
    /// Tipo de proveedor: "password", "google.com", "facebook.com", etc.
    #[serde(rename = "providerId")]
    pub provider_id: String,
    /// ID del usuario en el proveedor externo (ej: ID de Google)
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

/// Información adicional de Firebase en el token JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseInfo {
    /// Identidades vinculadas al usuario (ej: {"google.com": ["user@gmail.com"]})
    pub identities: HashMap<String, Vec<String>>,
    /// Proveedor usado en este login específico
    pub sign_in_provider: String,
}

/// Respuesta de Firebase Admin API al buscar usuarios
#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseAdminLookupResponse {
    pub users: Vec<FirebaseUserInfo>,
}

/// Usuario combinado con datos de Firebase Auth y nuestra base de datos
#[derive(Debug, Serialize)]
pub struct UserMerged {
    pub local_id: String,
    /// Indica si el usuario ya usó su clase gratuita de prueba
    pub first_free_class: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Rol del usuario en nuestra aplicación (desde nuestra DB, no Firebase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Tier de suscripción (desde nuestra DB, no Firebase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tier: Option<String>,
    /// Lista de permisos específicos (desde nuestra DB, no Firebase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_user_info: Option<Vec<ProviderUserInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_updated_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth: Option<bool>,
}

impl FirebaseAdminLookupResponse {
    /// Combina usuarios de Firebase Auth con datos de nuestra base de datos.
    /// Busca primero por local_id, si no encuentra intenta match por email.
    pub fn merge(self, db_users: HashMap<String, UserDB>) -> Vec<UserMerged> {
        self.users
            .into_iter()
            .map(|auth_user| {
                // Intentar encontrar usuario en nuestra DB por ID o email
                let db_user = db_users.get(&auth_user.local_id).or_else(|| {
                    auth_user
                        .email
                        .as_ref()
                        .and_then(|email| db_users.values().find(|db| db.email == *email))
                });

                UserMerged {
                    local_id: auth_user.local_id.clone(),
                    email: auth_user.email.clone(),
                    email_verified: auth_user.email_verified,
                    display_name: auth_user.display_name.clone(),
                    photo_url: auth_user.photo_url.clone(),
                    phone_number: auth_user.phone_number.clone(),
                    disabled: auth_user.disabled,
                    provider_user_info: auth_user.provider_user_info.clone(),
                    password_hash: auth_user.password_hash.clone(),
                    password_updated_at: auth_user.password_updated_at,
                    valid_since: auth_user.valid_since.clone(),
                    last_login_at: auth_user.last_login_at.clone(),
                    created_at: auth_user.created_at.clone(),
                    custom_auth: auth_user.custom_auth,
                    // Datos de nuestra DB (con defaults si no existe)
                    role: db_user.and_then(|db| db.role.clone()),
                    first_free_class: db_user.is_some_and(|db| db.first_free_class),
                    subscription_tier: db_user.and_then(|db| db.subscription_tier.clone()),
                    permissions: db_user
                        .and_then(|db| db.permissions.clone().map(|set| set.into_iter().collect())),
                }
            })
            .collect()
    }
}
