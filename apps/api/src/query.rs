use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state).await?;
        value.validate()?;
        Ok(ValidatedQuery(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Deserialize, Serialize, Validate)]
    struct TestPayload {
        #[validate(length(min = 3))]
        name: String,
    }

    #[tokio::test]
    async fn test_validated_query_success() {
        let mut req = Request::builder()
            .uri("/videos/upload-status?name=John")
            .body(Body::empty())
            .unwrap()
            .into_parts()
            .0;

        let result = ValidatedQuery::<TestPayload>::from_request_parts(&mut req, &()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.name, "John");
    }

    #[tokio::test]
    async fn test_validated_query_validation_failure() {
        let mut req = Request::builder()
            .uri("/videos/upload-status?name=Jo")
            .body(Body::empty())
            .unwrap()
            .into_parts()
            .0;

        let result = ValidatedQuery::<TestPayload>::from_request_parts(&mut req, &()).await;
        assert!(matches!(result, Err(AppError::ValidationErrors(_))));
    }
}
