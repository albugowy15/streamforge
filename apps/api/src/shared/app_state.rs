use std::sync::Arc;

use crate::modules::books::{BookRepository, BookService};

pub struct AppState {
    pub book_service: BookService,
}

impl AppState {
    pub fn new(book_repository: Arc<dyn BookRepository>) -> Self {
        Self {
            book_service: BookService::new(book_repository),
        }
    }
}
