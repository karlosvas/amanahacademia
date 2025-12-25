use {
    crate::services::firebase::handle_firebase_response,
    axum::http::StatusCode,
    serde::{Deserialize, Serialize},
};

// Estructura de prueba para deserializar respuestas exitosas
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct MockUser {
    id: String,
    email: String,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_successful_response_with_valid_json() {
        // Arrange: Crear un servidor mock
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": "123", "email": "test@example.com", "name": "Test User"}"#)
            .create();

        // Act: Hacer la petición y parsear con handle_firebase_response
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert: Verificar el resultado
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, "123");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");

        mock.assert();
    }

    #[tokio::test]
    async fn test_successful_response_with_invalid_json() {
        // Arrange: Respuesta exitosa pero con JSON inválido para el tipo esperado
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invalid": "data"}"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert: Debe fallar al parsear
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error_msg, "Error parsing Firebase response");

        mock.assert();
    }

    #[tokio::test]
    async fn test_error_response_with_firebase_error_format() {
        // Arrange: Simular un error de Firebase con formato estándar
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"message": "INVALID_EMAIL", "code": 400}}"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(error_msg, "INVALID_EMAIL");

        mock.assert();
    }

    #[tokio::test]
    async fn test_error_response_with_error_object_no_message() {
        // Arrange: Error de Firebase sin campo "message"
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"code": 403, "details": "Forbidden"}}"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert: Debe devolver el objeto error completo
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert!(error_msg.contains("code"));
        assert!(error_msg.contains("403"));

        mock.assert();
    }

    #[tokio::test]
    async fn test_error_response_without_error_field() {
        // Arrange: Error sin el campo "error" estándar de Firebase
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "Internal Server Error", "status": 500}"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert: Debe devolver todo el JSON
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(error_msg.contains("Internal Server Error"));

        mock.assert();
    }

    #[tokio::test]
    async fn test_error_response_with_non_json_body() {
        // Arrange: Error con respuesta no JSON
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(404)
            .with_header("content-type", "text/plain")
            .with_body("Not Found")
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert: Debe devolver el texto raw
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(error_msg, "Not Found");

        mock.assert();
    }

    #[tokio::test]
    async fn test_error_response_with_unknown_status_code() {
        // Arrange: Status code válido pero inusual
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(418) // I'm a teapot
            .with_body("I'm a teapot")
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::IM_A_TEAPOT);
        assert_eq!(error_msg, "I'm a teapot");

        mock.assert();
    }

    #[tokio::test]
    async fn test_successful_response_with_empty_object() {
        // Arrange: Respuesta exitosa con objeto vacío
        #[derive(Debug, Deserialize, Serialize)]
        struct EmptyResponse {}

        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/empty")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/empty", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<EmptyResponse, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert
        assert!(result.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_unauthorized_error() {
        // Arrange: Error de autenticación
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/user")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"message": "UNAUTHORIZED", "code": 401}}"#)
            .create();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user", server.url()))
            .send()
            .await
            .unwrap();

        let result: Result<MockUser, (StatusCode, String)> =
            handle_firebase_response(response).await;

        // Assert
        assert!(result.is_err());
        let (status, error_msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(error_msg, "UNAUTHORIZED");

        mock.assert();
    }
}
