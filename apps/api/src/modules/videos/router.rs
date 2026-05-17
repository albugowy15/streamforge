use std::sync::Arc;
use axum::{Router, routing::post};
use crate::state::AppState;
use super::controller::{create_video_handler, list_videos_handler};

pub struct VideosRouter;

impl VideosRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new()
            .route("/videos", post(create_video_handler).get(list_videos_handler))
    }
}
