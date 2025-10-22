use serde::Serialize;

/// Estructura genérica para respuestas de la API
#[derive(Serialize)]
pub struct ResponseAPI<T>
where
    T: Serialize,
{
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ResponseAPI<T>
where
    T: Serialize,
{
    pub fn success(message: String, data: T) -> Self {
        ResponseAPI {
            success: true,
            message: Some(message),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        ResponseAPI {
            success: false,
            message: None,
            data: None,
            error: Some(error),
        }
    }

    pub fn success_no_data() -> Self {
        ResponseAPI {
            success: true,
            message: Some("Operation successful".to_string()),
            data: None,
            error: None,
        }
    }
}
