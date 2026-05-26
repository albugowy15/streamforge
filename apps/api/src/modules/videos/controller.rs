use super::models::CreateVideoRequest;
use crate::{
    error::AppError,
    json::{AppJson, ValidatedJson},
    modules::videos::models::{CreateVideoResponseJson, UploadVideoChunkResponseJson},
    state::AppState,
};
use axum::extract::State;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/videos",
    request_body = CreateVideoRequest,
    responses(
        (status = 200, description = "Video created successfully", body = CreateVideoResponseJson),
        (status = 400, description = "Invalid input")
    ),
    tag = "Videos"
)]
pub async fn create_video_handler(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateVideoRequest>,
) -> Result<CreateVideoResponseJson, AppError> {
    Ok(AppJson(state.videos_service.create(payload).await?.into()))
}

#[utoipa::path(
    post,
    path = "/videos/{id}/parts",
    responses(
        (status = 200, description = "Chunk uploaded successfully", body = String)
    ),
    tag = "Videos"
)]
pub async fn upload_video_chunk_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<UploadVideoChunkResponseJson, AppError> {
    Ok(AppJson("Success".to_string()))
}

#[utoipa::path(
    get,
    path = "/videos/{id}/upload-status",
    responses(
        (status = 200, description = "Upload status retrieved", body = String)
    ),
    tag = "Videos"
)]
pub async fn show_upload_status_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<AppJson<String>, AppError> {
    Ok(AppJson("Success".to_string()))
}
