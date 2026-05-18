use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::json::AppJson;

#[derive(Debug)]
pub enum AppError {
    JsonRejection(JsonRejection),
    Internal(String),
    Unathorized,
    BadRequest(Vec<String>),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
            errors: Vec<String>,
        }
        let (status, err) = match &self {
            AppError::JsonRejection(rejection) => (
                rejection.status(),
                ErrorResponse {
                    message: rejection.body_text(),
                    errors: vec![rejection.body_text()],
                },
            ),
            AppError::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    message: message.clone(),
                    errors: vec![message.clone()],
                },
            ),
            AppError::Unathorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    message: StatusCode::UNAUTHORIZED.to_string(),
                    errors: vec![StatusCode::UNAUTHORIZED.to_string()],
                },
            ),
            AppError::BadRequest(errors) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: StatusCode::BAD_REQUEST.to_string(),
                    errors: errors.clone(),
                },
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    message: StatusCode::NOT_FOUND.to_string(),
                    errors: vec![StatusCode::NOT_FOUND.to_string()],
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
