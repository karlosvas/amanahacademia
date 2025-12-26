#[cfg(test)]
mod tests {
    use {
        crate::{
            models::metrics::{GAErrorResponse, GAResponse},
            services::metrics::parse_ga_response,
        },
        mockito::Server,
    };

    #[tokio::test]
    async fn test_parse_ga_response_with_success() {
        // Arrange: Crear un servidor mock con una respuesta exitosa
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/analytics")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"rows": []}"#)
            .create();

        // Act: Hacer la petición
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/analytics", server.url()))
            .send()
            .await;

        let result: Result<GAResponse, _> = parse_ga_response(response).await;

        // Assert: Verificar que la respuesta fue exitosa
        assert!(result.is_ok());
        let ga_response = result.unwrap();
        assert_eq!(ga_response.rows.len(), 0);

        mock.assert();
    }

    #[tokio::test]
    async fn test_parse_ga_response_with_data() {
        // Arrange: Crear un servidor mock con datos de métricas
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/analytics")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "rows": [
                        {
                            "dimensionValues": [{"value": "202412"}],
                            "metricValues": [{"value": "1234"}]
                        }
                    ]
                }"#,
            )
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/analytics", server.url()))
            .send()
            .await;

        let result: Result<GAResponse, _> = parse_ga_response(response).await;

        // Assert
        assert!(result.is_ok());
        let ga_response = result.unwrap();
        assert_eq!(ga_response.rows.len(), 1);

        mock.assert();
    }

    #[tokio::test]
    async fn test_parse_ga_response_with_ga_error() {
        // Arrange: Simular un error de Google Analytics
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/analytics")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "error": {
                        "code": 400,
                        "message": "Invalid request",
                        "status": "INVALID_ARGUMENT"
                    }
                }"#,
            )
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/analytics", server.url()))
            .send()
            .await;

        let result: Result<GAResponse, _> = parse_ga_response(response).await;

        // Assert: Debe fallar con error de GA
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid request"));

        mock.assert();
    }

    #[tokio::test]
    async fn test_parse_ga_response_with_invalid_json() {
        // Arrange: Respuesta exitosa pero con JSON completamente inválido
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/analytics")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"not a valid json at all"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/analytics", server.url()))
            .send()
            .await;

        let result: Result<GAResponse, _> = parse_ga_response(response).await;

        // Assert: Debe fallar al parsear
        assert!(result.is_err());

        mock.assert();
    }

    #[tokio::test]
    async fn test_parse_ga_response_with_network_error() {
        // Act: Intentar conectar a un servidor que no existe
        let client = reqwest::Client::new();
        let response = client
            .get("http://invalid-url-that-does-not-exist")
            .send()
            .await;

        let result: Result<GAResponse, _> = parse_ga_response(response).await;

        // Assert: Debe fallar con error de red
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Network error"));
    }

    #[tokio::test]
    async fn test_parse_ga_response_with_non_success_status() {
        // Arrange: Respuesta con código de error sin formato GA
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/analytics")
            .with_status(500)
            .with_header("content-type", "text/plain")
            .with_body("Internal Server Error")
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/analytics", server.url()))
            .send()
            .await;

        let result: Result<GAResponse, _> = parse_ga_response(response).await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Internal Server Error"));

        mock.assert();
    }

    #[tokio::test]
    async fn test_parse_ga_response_with_unauthorized() {
        // Arrange: Error de autenticación
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/analytics")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "error": {
                        "code": 401,
                        "message": "Request is missing required authentication credential",
                        "status": "UNAUTHENTICATED"
                    }
                }"#,
            )
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/analytics", server.url()))
            .send()
            .await;

        let result: Result<GAErrorResponse, _> = parse_ga_response(response).await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("authentication"));

        mock.assert();
    }
}
