use super::controller::*;
use crate::state::AppState;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub struct VideosRouter;

impl VideosRouter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> OpenApiRouter<Arc<AppState>> {
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(create_video_handler))
            .routes(utoipa_axum::routes!(upload_video_part_handler))
            .routes(utoipa_axum::routes!(show_upload_status_handler))
            .routes(utoipa_axum::routes!(complete_video_upload_handler))
            .routes(utoipa_axum::routes!(abort_video_upload_handler))
    }
}
