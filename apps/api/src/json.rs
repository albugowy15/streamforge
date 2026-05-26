use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use crate::error::AppError;

#[derive(FromRequest, utoipa::ToSchema)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct JsonData<T: Serialize> {
    pub data: T,
}

impl<T> From<JsonData<T>> for AppJson<JsonData<T>>
where
    T: Serialize,
{
    fn from(value: JsonData<T>) -> Self {
        AppJson(value)
    }
}

impl<T> From<Vec<T>> for JsonData<Vec<T>>
where
    T: Serialize,
{
    fn from(value: Vec<T>) -> Self {
        JsonData { data: value }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, header};
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Serialize, Validate)]
    struct TestPayload {
        #[validate(length(min = 3))]
        name: String,
    }

    #[tokio::test]
    async fn test_validated_json_success() {
        let payload = TestPayload {
            name: "John".to_string(),
        };
        let req = Request::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_vec(&payload).unwrap(),
            ))
            .unwrap();

        let result = ValidatedJson::<TestPayload>::from_request(req, &()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.name, "John");
    }

    #[tokio::test]
    async fn test_validated_json_validation_failure() {
        let payload = TestPayload {
            name: "Jo".to_string(),
        };
        let req = Request::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_vec(&payload).unwrap(),
            ))
            .unwrap();

        let result = ValidatedJson::<TestPayload>::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::ValidationErrors(_))));
    }

    #[tokio::test]
    async fn test_validated_json_json_rejection() {
        let req = Request::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from("invalid json"))
            .unwrap();

        let result = ValidatedJson::<TestPayload>::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::JsonRejection(_))));
    }

    #[test]
    fn test_json_data_conversions() {
        let items = vec![1, 2, 3];
        let json_data: JsonData<Vec<i32>> = items.into();
        assert_eq!(json_data.data, vec![1, 2, 3]);

        let app_json: AppJson<JsonData<Vec<i32>>> = json_data.into();
        assert_eq!(app_json.0.data, vec![1, 2, 3]);
    }
}
