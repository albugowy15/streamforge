use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

use crate::json::AppJson;

#[derive(Debug)]
pub enum AppError {
    ValidationErrors(ValidationErrors),
    JsonRejection(JsonRejection),
    Internal(String),
    Unathorized,
    BadRequest(String),
    NotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: Option<String>,
    errors: Option<ValidationErrors>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err) = match self {
            AppError::ValidationErrors(errors) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: Some(StatusCode::BAD_REQUEST.to_string()),
                    errors: Some(errors),
                },
            ),
            AppError::JsonRejection(rejection) => (
                rejection.status(),
                ErrorResponse {
                    message: Some(rejection.body_text()),
                    errors: None,
                },
            ),
            AppError::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    message: Some(message.to_owned()),
                    errors: None,
                },
            ),
            AppError::Unathorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    message: Some(StatusCode::UNAUTHORIZED.to_string()),
                    errors: None,
                },
            ),
            AppError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: Some(message.to_owned()),
                    errors: None,
                },
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    message: Some(StatusCode::NOT_FOUND.to_string()),
                    errors: None,
                },
            ),
        };
        (status, AppJson(err)).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(value: ValidationErrors) -> Self {
        Self::ValidationErrors(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use validator::ValidationError;

    #[tokio::test]
    async fn test_not_found_error() {
        let err = AppError::NotFound;
        let res = err.into_response();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = to_bytes(res.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "404 Not Found");
    }

    #[tokio::test]
    async fn test_internal_error() {
        let err = AppError::Internal("something went wrong".to_string());
        let res = err.into_response();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = to_bytes(res.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "something went wrong");
    }

    #[tokio::test]
    async fn test_validation_errors() {
        let mut errors = ValidationErrors::new();
        errors.add("email", ValidationError::new("invalid_email"));

        let err = AppError::ValidationErrors(errors);
        let res = err.into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(res.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "400 Bad Request");
        assert!(json["errors"]["email"].is_array());
        assert_eq!(json["errors"]["email"][0]["code"], "invalid_email");
    }

    #[tokio::test]
    async fn test_unauthorized_error() {
        let err = AppError::Unathorized;
        let res = err.into_response();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = to_bytes(res.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "401 Unauthorized");
    }

    #[tokio::test]
    async fn test_bad_request_error() {
        let err = AppError::BadRequest("invalid input".to_string());
        let res = err.into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(res.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "invalid input");
    }
}
