use std::sync::Arc;
use axum::{Json, extract::State, http::StatusCode};
use crate::state::AppState;
use super::models::{CreateVideoRequest, VideoResponse};

pub async fn create_video_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateVideoRequest>,
) -> Result<(StatusCode, Json<VideoResponse>), (StatusCode, String)> {
    state
        .videos_service
        .create(payload)
        .await
        .map(|res| (StatusCode::CREATED, Json(res)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn list_videos_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<VideoResponse>>, (StatusCode, String)> {
    state
        .videos_service
        .list()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
