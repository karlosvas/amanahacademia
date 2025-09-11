use {axum::http::StatusCode, serde::de::DeserializeOwned};

/// Maneja la respuesta de Firebase
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

        // Si el error es JSON, intenta extraer el campo "error" como texto
        let error_msg = match serde_json::from_str::<serde_json::Value>(&error_text) {
            Ok(json) => json
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or(&error_text)
                .to_string(),
            Err(_) => error_text,
        };

        Err((status, error_msg))
    }
}
