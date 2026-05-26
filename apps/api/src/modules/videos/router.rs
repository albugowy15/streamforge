use super::controller::{
    create_video_handler, show_upload_status_handler, upload_video_chunk_handler,
};
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub struct VideosRouter;

impl VideosRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new()
            .route("/videos", post(create_video_handler))
            .route("/videos/{id}/parts", post(upload_video_chunk_handler))
            .route(
                "/videos/{id}/upload-status",
                get(show_upload_status_handler),
            )
    }
}
