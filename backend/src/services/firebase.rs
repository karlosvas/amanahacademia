use {
    axum::http::StatusCode,
    serde::de::DeserializeOwned,
    serde_json::{Value, json},
};

/// Maneja la respuesta de Firebase
pub async fn handle_firebase_response<T>(
    response: reqwest::Response,
) -> Result<T, (StatusCode, Value)>
where
    T: DeserializeOwned,
{
    // Verifica si la respuesta es exitosa
    if response.status().is_success() {
        match response.json::<T>().await {
            Ok(parsed_response) => Ok(parsed_response),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "Error parsing Firebase response" }),
            )),
        }
    } else {
        let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let error_text: String = response.text().await.unwrap_or_default();

        // Intenta deserializar el error como JSON
        let error_json: Value = match serde_json::from_str(&error_text) {
            Ok(json) => json,
            Err(_) => json!({ "error": error_text }),
        };

        Err((status, error_json))
    }
}
