use {
    serde::{Deserialize, Serialize},
    std::collections::HashSet,
};

// Estructura de los datos que se pasan desde el frontend para los enpoints de users
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    // Datos obligatorios requeridos por firebase auth
    pub email: String,
    pub password: String,
    pub provider: Provider, // "email" o "google"

    // Datos opcionales que el cliente puede enviar
    pub name: Option<String>, // Solo necesario en los register o en los update
    pub phone_number: Option<String>,
    pub id_token: Option<String>, // Token JWT de Firebase Auth

    // Datos especificos para la DB
    pub role: Option<ROLE>,
    pub permissions: Option<HashSet<String>>,
    pub subscription_tier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Email,
    Google,
}

impl Provider {
    /// Returns the string representation used by the API/frontend for this provider
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::Email => "email",
            Provider::Google => "google",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ROLE {
    ADMIN,
    STUDENT,
    TEACHER,
}
impl PartialEq<str> for ROLE {
    fn eq(&self, other: &str) -> bool {
        match self {
            ROLE::ADMIN => other == "admin",
            ROLE::STUDENT => other == "student",
            ROLE::TEACHER => other == "teacher",
        }
    }
}

impl std::fmt::Display for ROLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ROLE::ADMIN => "admin",
            ROLE::STUDENT => "student",
            ROLE::TEACHER => "teacher",
        };
        write!(f, "{}", s)
    }
}

// Usuario de la base de datos
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserDB {
    pub email: String,
    pub role: Option<String>,
    pub subscription_tier: Option<String>,
    pub permissions: Option<HashSet<String>>,
}
