use validator::ValidationError;

pub fn validate_non_whitespace(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("cannot_be_empty"));
    }
    Ok(())
}

/// Wrapper que valida automáticamente para Axum
pub struct ValidatedJson<T>(pub T);

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
