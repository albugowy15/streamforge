use crate::{
    modules::books::controller::{
        create_book_handler, delete_book_handler, get_book_handler, list_books_handler,
        update_book_handler,
    },
    shared::app_state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub struct BookRouter;

impl BookRouter {
    pub fn new() -> Router<Arc<AppState>> {
        Router::new()
            .route("/books", post(create_book_handler).get(list_books_handler))
            .route(
                "/books/{id}",
                get(get_book_handler)
                    .put(update_book_handler)
                    .delete(delete_book_handler),
            )
    }
}
