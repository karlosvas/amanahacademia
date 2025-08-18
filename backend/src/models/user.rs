use {
    serde::{Deserialize, Serialize},
    std::collections::HashSet,
};

/// Estructuras de usuario personalizadas

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

// Usuario de la base de datos
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserDB {
    pub email: String,
    pub role: Option<String>,
    pub subscription_tier: Option<String>,
    pub permissions: Option<HashSet<String>>,
}
