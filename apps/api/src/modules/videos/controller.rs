use super::models::CreateVideoRequest;
use crate::{
    error::AppError,
    json::{AppJson, ValidatedJson},
    modules::videos::models::CreateVideoResponse,
    state::AppState,
};
use axum::extract::State;
use std::sync::Arc;

pub async fn create_video_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateVideoRequest>,
) -> Result<CreateVideoResponse, AppError> {
    let res = state
        .videos_service
        .create(payload)
        .await
        .map_err(AppError::Internal)?;

    Ok(AppJson(res.into()))
}
