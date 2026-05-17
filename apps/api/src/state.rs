use crate::modules::{books::BookService, videos::VideosService};

pub struct AppState {
    pub book_service: BookService,
    pub videos_service: VideosService,
}
