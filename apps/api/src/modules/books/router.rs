use crate::{modules::books::controller::*, state::AppState};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub struct BookRouter;

impl BookRouter {
    pub fn new() -> OpenApiRouter<Arc<AppState>> {
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(
                create_book_handler,
                list_books_handler
            ))
            .routes(utoipa_axum::routes!(
                get_book_handler,
                update_book_handler,
                delete_book_handler
            ))
    }
}
