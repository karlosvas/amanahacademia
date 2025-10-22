use {
    serde::{Deserialize, Serialize},
    std::collections::HashSet,
};

/// Estructura de los datos que se pasan desde el frontend para los enpoints de users
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    // Datos obligatorios requeridos por Firebase Auth
    pub email: String,
    pub password: String,
    pub provider: Provider,     // "email" o "google"
    pub first_free_class: bool, // Indica si es la primera clase gratis del usuario

    // Datos opcionales que el cliente puede enviar
    pub name: Option<String>, // Solo necesario en los register o en los update
    pub phone_number: Option<String>,
    pub id_token: Option<String>, // Token JWT de Firebase Auth

    // Datos especificos para la DB
    pub role: Option<Role>,
    pub permissions: Option<HashSet<String>>,
    pub subscription_tier: Option<String>,
}

/// Proveedor de autenticación usado por el usuario
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Email,
    Google,
}

/// Roles posibles de un usuario en la aplicación
/// Roles posibles de un usuario en la aplicación
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Student,
    Teacher,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Role::Admin => "admin",
            Role::Student => "student",
            Role::Teacher => "teacher",
        };
        write!(f, "{}", s)
    }
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::Student => "student",
            Role::Teacher => "teacher",
        }
    }
}

/// Usuario de la base de datos
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserDB {
    pub email: String,
    pub first_free_class: bool,
    pub role: Option<String>,
    pub subscription_tier: Option<String>,
    pub permissions: Option<HashSet<String>>,
}
