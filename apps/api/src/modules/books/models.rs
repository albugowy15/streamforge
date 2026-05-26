use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// --- Entities (Domain) ---
#[derive(Debug, Clone, ToSchema)]
pub struct Book {
    pub id: Option<i64>,
    pub title: String,
    pub authors: Vec<String>,
    pub publishers: Vec<String>,
    pub date_published: NaiveDate,
    pub abstract_text: String,
}

impl Book {
    pub fn new(
        id: Option<i64>,
        title: String,
        authors: Vec<String>,
        publishers: Vec<String>,
        date_published: NaiveDate,
        abstract_text: String,
    ) -> Self {
        Self {
            id,
            title,
            authors,
            publishers,
            date_published,
            abstract_text,
        }
    }
}

// --- DTOs (Use Case / Interface) ---
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBookRequest {
    pub title: String,
    pub authors: Vec<String>,
    pub publishers: Vec<String>,
    pub date_published: NaiveDate,
    pub abstract_text: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateBookRequest {
    pub id: i64,
    pub title: String,
    pub authors: Vec<String>,
    pub publishers: Vec<String>,
    pub date_published: NaiveDate,
    pub abstract_text: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookResponse {
    pub id: i64,
    pub title: String,
    pub authors: Vec<String>,
    pub publishers: Vec<String>,
    pub date_published: NaiveDate,
    pub abstract_text: String,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        Self {
            id: book.id.unwrap_or(0),
            title: book.title,
            authors: book.authors,
            publishers: book.publishers,
            date_published: book.date_published,
            abstract_text: book.abstract_text,
        }
    }
}
