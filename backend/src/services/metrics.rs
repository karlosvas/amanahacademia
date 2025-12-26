use {
    crate::models::{error::MetricsError, metrics::GAErrorResponse},
    serde::de::DeserializeOwned,
};

pub async fn parse_ga_response<T>(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<T, MetricsError>
where
    T: DeserializeOwned,
{
    match response {
        Ok(parsed_response) => {
            // Verifica si la respuesta es exitosa
            if parsed_response.status().is_success() {
                match parsed_response.json::<T>().await {
                    Ok(parsed_response) => Ok(parsed_response),
                    Err(e) => Err(MetricsError::Network(e)),
                }
            } else {
                let error_text: String = parsed_response.text().await.unwrap_or_default();

                // Intenta parsear el JSON del error de Google analitics
                if let Ok(ga_error) = serde_json::from_str::<GAErrorResponse>(&error_text) {
                    return Err(ga_error.into());
                }

                Err(MetricsError::ApiText(error_text))
            }
        }
        Err(e) => Err(MetricsError::Network(e)),
    }
}

#[cfg(test)]
#[path = "../test/services/metrics.rs"]
mod tests;
