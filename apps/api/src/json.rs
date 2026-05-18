use axum::{
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::error::AppError;

#[derive(FromRequest)]
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

#[derive(Serialize)]
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
