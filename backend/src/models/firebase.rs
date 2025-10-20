use {
    crate::models::user::UserDB,
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

/// Estructuras de usuario que vienen por defecto en Firebase (no tocar)
// Token de actualización
#[derive(Deserialize, Serialize)]
pub struct RefreshToken {
    pub grant_type: String,
    pub refresh_token: String,
}

// Usuario en Auth
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserAuth {
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    pub email: String,
    pub password: String,
    pub return_secure_token: bool,
    pub display_name: Option<String>,
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

// Info de firebase
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseInfo {
    pub identities: HashMap<String, Vec<String>>,
    pub sign_in_provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseAdminLookupResponse {
    pub users: Vec<FirebaseUserInfo>,
}

#[derive(Debug, Serialize)]
pub struct UserMerged {
    // Obligatorios
    pub local_id: String,
    pub first_free_class: bool,
    // Opcionales
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tier: Option<String>,
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
    pub fn merge(self, db_users: HashMap<String, UserDB>) -> Vec<UserMerged> {
        self.users
            .into_iter()
            .map(|auth_user| {
                // Busca el usuario en la DB por local_id o email
                let db_user = db_users.get(&auth_user.local_id).or_else(|| {
                    auth_user
                        .email
                        .as_ref()
                        .and_then(|email| db_users.values().find(|db| db.email == *email))
                });
                UserMerged {
                    // Campos de Auth
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
                    // Campos de DB
                    role: db_user.and_then(|db| db.role.clone()),
                    first_free_class: db_user.map_or(false, |db| db.first_free_class),
                    subscription_tier: db_user.and_then(|db| db.subscription_tier.clone()),
                    permissions: db_user
                        .and_then(|db| db.permissions.clone().map(|set| set.into_iter().collect())),
                }
            })
            .collect()
    }
}
