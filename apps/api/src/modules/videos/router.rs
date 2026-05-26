use super::controller::*;
use crate::state::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub struct VideosRouter;

impl VideosRouter {
    pub fn new() -> OpenApiRouter<Arc<AppState>> {
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(create_video_handler))
            .routes(utoipa_axum::routes!(upload_video_chunk_handler))
            .routes(utoipa_axum::routes!(show_upload_status_handler))
    }
}
