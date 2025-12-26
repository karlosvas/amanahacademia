use {
    async_trait::async_trait,
    axum::{
        extract::{FromRequest, Request},
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
    validator::{Validate, ValidationError},
};

/// Valida que una cadena no esté vacía o compuesta solo por espacios en blanco
pub fn validate_non_whitespace(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("cannot_be_empty"));
    }
    Ok(())
}

/// Wrapper que valida automáticamente para Axum
pub struct ValidatedJson<T>(pub T);

/// Implementación de FromRequest para ValidatedJson
#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: for<'de> serde::Deserialize<'de> + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)).into_response()
        })?;

        data.validate().map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("Validation error: {}", e)).into_response()
        })?;

        Ok(ValidatedJson(data))
    }
}

#[cfg(test)]
#[path = "../test/validations/validations.rs"]
mod tests;
