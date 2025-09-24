use axum::http::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Maneja la respuesta de Mailchimp
pub async fn handle_mailchimp_response<T>(
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
                "Error parsing Mailchimp response".to_string(),
            )),
        }
    } else {
        let status: StatusCode = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let error_text: String = response.text().await.unwrap_or_default();

        // Intenta parsear el JSON de error de Mailchimp
        let error_msg = match serde_json::from_str::<Value>(&error_text) {
            Ok(json) => {
                // Busca el campo "title" en el JSON raíz
                let res = json
                    .get("title")
                    .and_then(|title| title.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or(error_text);
                // Si no es JSON válido, devuelve el texto raw
                res
            }
            Err(_) => error_text,
        };

        Err((status, error_msg))
    }
}

// EJEMPLO
/*
{
    "success":false,
    "error": {
        "Mailchimp API error (400 Bad Request)": {
            "title":"Member Exists","status":400,"detail":"tayeteg210@anysilo.com is already a list member. Use PUT to insert or update list members.","instance":"ac48260b-2e02-1034-d21a-d3a2061dd795"
        }
    }
}
*/
