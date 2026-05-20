use axum::{
    Form,
    extract::{FromRequest, rejection::FormRejection},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, header};
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Deserialize, Serialize, Validate)]
    struct TestPayload {
        #[validate(length(min = 3))]
        name: String,
    }

    #[tokio::test]
    async fn test_validated_form_success() {
        let req = Request::builder()
            .method("POST")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(axum::body::Body::from("name=John"))
            .unwrap();

        let result = ValidatedForm::<TestPayload>::from_request(req, &()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.name, "John");
    }

    #[tokio::test]
    async fn test_validated_form_validation_failure() {
        let req = Request::builder()
            .method("POST")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(axum::body::Body::from("name=Jo"))
            .unwrap();

        let result = ValidatedForm::<TestPayload>::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::ValidationErrors(_))));
    }

    #[tokio::test]
    async fn test_validated_form_rejection() {
        let req = Request::builder()
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from("name=John"))
            .unwrap();

        let result = ValidatedForm::<TestPayload>::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::FormRejection(_))));
    }
}
