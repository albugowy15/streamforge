use super::models::{
    CompleteVideoUploadRequest, CreateVideoRequest, UploadVideoPartResponseJson, UploadVideoQuery,
};
use crate::{
    error::AppError,
    json::{AppJson, ValidatedJson},
    modules::videos::models::{
        AbortVideoUploadResponseJson, CompleteVideoUploadResponseJson, CreateVideoResponseJson,
        UploadVideoStatusResponseJson,
    },
    query::ValidatedQuery,
    state::AppState,
};
use axum::{
    body::Bytes,
    extract::{Path, State},
};
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
    put,
    path = "/videos/{id}/parts/{part_number}",
    params(
        ("id" = String, Path, description = "Video id returned by POST /videos"),
        ("part_number" = i32, Path, description = "S3 multipart part number from 1 to 10000"),
        ("upload_id" = String, Query, description = "S3 multipart upload id returned by POST /videos")
    ),
    request_body(content = Vec<u8>, content_type = "application/octet-stream"),
    responses(
        (status = 200, description = "Part uploaded successfully", body = UploadVideoPartResponseJson),
        (status = 400, description = "Invalid upload id, part number, or body")
    ),
    tag = "Videos"
)]
pub async fn upload_video_part_handler(
    State(state): State<Arc<AppState>>,
    Path((id, part_number)): Path<(String, i32)>,
    ValidatedQuery(query): ValidatedQuery<UploadVideoQuery>,
    body: Bytes,
) -> Result<UploadVideoPartResponseJson, AppError> {
    Ok(AppJson(
        state
            .videos_service
            .upload_part(id, query.upload_id, part_number, body.to_vec())
            .await?
            .into(),
    ))
}

#[utoipa::path(
    get,
    path = "/videos/{id}/upload-status",
    params(
        ("id" = String, Path, description = "Video id returned by POST /videos"),
        ("upload_id" = String, Query, description = "S3 multipart upload id returned by POST /videos")
    ),
    responses(
        (status = 200, description = "Upload status retrieved", body = UploadVideoStatusResponseJson)
    ),
    tag = "Videos"
)]
pub async fn show_upload_status_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ValidatedQuery(query): ValidatedQuery<UploadVideoQuery>,
) -> Result<UploadVideoStatusResponseJson, AppError> {
    Ok(AppJson(
        state
            .videos_service
            .upload_status(id, query.upload_id)
            .await?
            .into(),
    ))
}

#[utoipa::path(
    post,
    path = "/videos/{id}/complete-upload",
    params(
        ("id" = String, Path, description = "Video id returned by POST /videos")
    ),
    request_body = CompleteVideoUploadRequest,
    responses(
        (status = 200, description = "Upload completed successfully", body = CompleteVideoUploadResponseJson),
        (status = 400, description = "Invalid upload id or uploaded parts")
    ),
    tag = "Videos"
)]
pub async fn complete_video_upload_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ValidatedJson(payload): ValidatedJson<CompleteVideoUploadRequest>,
) -> Result<CompleteVideoUploadResponseJson, AppError> {
    Ok(AppJson(
        state
            .videos_service
            .complete_upload(id, payload)
            .await?
            .into(),
    ))
}

#[utoipa::path(
    delete,
    path = "/videos/{id}/upload",
    params(
        ("id" = String, Path, description = "Video id returned by POST /videos"),
        ("upload_id" = String, Query, description = "S3 multipart upload id returned by POST /videos")
    ),
    responses(
        (status = 200, description = "Upload aborted successfully", body = AbortVideoUploadResponseJson)
    ),
    tag = "Videos"
)]
pub async fn abort_video_upload_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ValidatedQuery(query): ValidatedQuery<UploadVideoQuery>,
) -> Result<AbortVideoUploadResponseJson, AppError> {
    Ok(AppJson(
        state
            .videos_service
            .abort_upload(id, query.upload_id)
            .await?
            .into(),
    ))
}
