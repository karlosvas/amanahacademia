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

    // Datos opcionales que el cliente puede enviar
    pub name: Option<String>,
    pub phone_number: Option<String>,

    // Datos especificos para la DB
    pub role: Option<ROLE>,
    pub permissions: Option<HashSet<String>>,
    pub subscription_tier: Option<String>,
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
// impl ROLE {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             ROLE::ADMIN => "admin",
//             ROLE::STUDENT => "student",
//             ROLE::TEACHER => "teacher",
//         }
//     }
//     pub fn from_str(s: &str) -> Option<ROLE> {
//         match s {
//             "admin" => Some(ROLE::ADMIN),
//             "student" => Some(ROLE::STUDENT),
//             "teacher" => Some(ROLE::TEACHER),
//             _ => None,
//         }
//     }
// }

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
