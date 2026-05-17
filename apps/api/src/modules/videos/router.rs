use super::controller::{create_video_handler, list_videos_handler};
use crate::state::AppState;
use axum::{Router, routing::post};
use std::sync::Arc;

pub struct VideosRouter;

impl VideosRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new().route(
            "/videos",
            post(create_video_handler).get(list_videos_handler),
        )
    }
}
