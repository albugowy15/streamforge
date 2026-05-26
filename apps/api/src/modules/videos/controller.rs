use super::models::CreateVideoRequest;
use crate::{
    error::AppError,
    json::{AppJson, ValidatedJson},
    modules::videos::models::{CreateVideoResponseJson, UploadVideoChunkResponseJson},
    state::AppState,
};
use axum::extract::State;
use std::sync::Arc;

pub async fn create_video_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateVideoRequest>,
) -> Result<CreateVideoResponseJson, AppError> {
    Ok(AppJson(state.videos_service.create(payload).await?.into()))
}

pub async fn upload_video_chunk_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<UploadVideoChunkResponseJson, AppError> {
    Ok(AppJson("Success".to_string()))
}

pub async fn show_upload_status_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<AppJson<String>, AppError> {
    Ok(AppJson("Success".to_string()))
}
