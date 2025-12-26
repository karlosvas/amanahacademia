use {axum::http::StatusCode, serde::de::DeserializeOwned, serde_json::Value};

/// Maneja la respuesta de Firebase, serializandolo
pub async fn handle_firebase_response<T>(
    response: reqwest::Response,
) -> Result<T, (StatusCode, String)>
where
    T: DeserializeOwned,
{
    // Verifica si la respuesta es exitosa
    if response.status().is_success() {
        match response.json::<T>().await {
            Ok(parsed_response) => Ok(parsed_response),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error parsing Firebase response".to_string(),
            )),
        }
    } else {
        let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let error_text: String = response.text().await.unwrap_or_default();

        // Intenta parsear el JSON de error de Firebase
        let error_msg: String = match serde_json::from_str::<Value>(&error_text) {
            Ok(json) => {
                // Firebase devuelve errores en formato: {"error": {"message": "...", "code": 400}}
                if let Some(error_obj) = json.get("error") {
                    if let Some(message) = error_obj.get("message") {
                        message.as_str().unwrap_or(&error_text).to_string()
                    } else {
                        // Si no hay message, devuelve el objeto error completo como string
                        error_obj.to_string()
                    }
                } else {
                    // Si no hay campo "error", devuelve todo el JSON
                    error_text
                }
            }
            Err(_) => {
                // Si no es JSON válido, devuelve el texto raw
                error_text
            }
        };

        Err((status, error_msg))
    }
}

#[cfg(test)]
#[path = "../test/services/firebase.rs"]
mod tests;
