use crate::modules::books::models::{Book, BookResponse, CreateBookRequest, UpdateBookRequest};
use crate::modules::books::repository::BookRepository;
use std::sync::Arc;

pub struct BookService {
    repository: Arc<dyn BookRepository>,
}

impl BookService {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, req: CreateBookRequest) -> Result<BookResponse, String> {
        let book = Book::new(
            None,
            req.title,
            req.authors,
            req.publishers,
            req.date_published,
            req.abstract_text,
        );
        let created_book = self.repository.create(book).await?;
        Ok(BookResponse::from(created_book))
    }

    pub async fn get(&self, id: i64) -> Result<Option<BookResponse>, String> {
        let book = self.repository.get_by_id(id).await?;
        Ok(book.map(BookResponse::from))
    }

    pub async fn update(&self, req: UpdateBookRequest) -> Result<BookResponse, String> {
        let book = Book::new(
            Some(req.id),
            req.title,
            req.authors,
            req.publishers,
            req.date_published,
            req.abstract_text,
        );
        let updated_book = self.repository.update(book).await?;
        Ok(BookResponse::from(updated_book))
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        self.repository.delete(id).await
    }

    pub async fn list(&self) -> Result<Vec<BookResponse>, String> {
        let books = self.repository.list().await?;
        Ok(books.into_iter().map(BookResponse::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::books::repository::BookRepository;
    use async_trait::async_trait;
    use chrono::NaiveDate;

    struct MockBookRepository;

    #[async_trait]
    impl BookRepository for MockBookRepository {
        async fn create(&self, book: Book) -> Result<Book, String> {
            let mut created_book = book;
            created_book.id = Some(1);
            Ok(created_book)
        }
        async fn get_by_id(&self, _id: i64) -> Result<Option<Book>, String> {
            Ok(None)
        }
        async fn update(&self, book: Book) -> Result<Book, String> {
            Ok(book)
        }
        async fn delete(&self, _id: i64) -> Result<(), String> {
            Ok(())
        }
        async fn list(&self) -> Result<Vec<Book>, String> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_create_book() {
        let repo = Arc::new(MockBookRepository);
        let service = BookService::new(repo);

        let req = CreateBookRequest {
            title: "Test Book".to_string(),
            authors: vec!["Author".to_string()],
            publishers: vec!["Publisher".to_string()],
            date_published: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            abstract_text: "Abstract".to_string(),
        };

        let res = service.create(req).await.unwrap();
        assert_eq!(res.id, 1);
    }
}
