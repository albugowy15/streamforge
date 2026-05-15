use crate::modules::books::models::Book;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{FromRow, PgPool};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, book: Book) -> Result<Book, String>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Book>, String>;
    async fn update(&self, book: Book) -> Result<Book, String>;
    async fn delete(&self, id: i64) -> Result<(), String>;
    async fn list(&self) -> Result<Vec<Book>, String>;
}

#[derive(FromRow)]
struct BookRow {
    id: i64,
    title: String,
    authors: Vec<String>,
    publishers: Vec<String>,
    date_published: NaiveDate,
    abstract_text: String,
}

impl From<BookRow> for Book {
    fn from(row: BookRow) -> Self {
        Self::new(
            Some(row.id),
            row.title,
            row.authors,
            row.publishers,
            row.date_published,
            row.abstract_text,
        )
    }
}

pub struct PostgresBookRepository {
    pool: PgPool,
}

impl PostgresBookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookRepository for PostgresBookRepository {
    async fn create(&self, book: Book) -> Result<Book, String> {
        let row = sqlx::query_as::<_, BookRow>(
            r#"
            INSERT INTO books (title, authors, publishers, date_published, abstract_text)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, authors, publishers, date_published, abstract_text
            "#,
        )
        .bind(book.title)
        .bind(&book.authors)
        .bind(&book.publishers)
        .bind(book.date_published)
        .bind(book.abstract_text)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Book::from(row))
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<Book>, String> {
        let row = sqlx::query_as::<_, BookRow>(
            r#"
            SELECT id, title, authors, publishers, date_published, abstract_text
            FROM books
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(Book::from))
    }

    async fn update(&self, book: Book) -> Result<Book, String> {
        let id = book
            .id
            .ok_or("Book ID is required for update".to_string())?;
        let row = sqlx::query_as::<_, BookRow>(
            r#"
            UPDATE books
            SET title = $1, authors = $2, publishers = $3, date_published = $4, abstract_text = $5
            WHERE id = $6
            RETURNING id, title, authors, publishers, date_published, abstract_text
            "#,
        )
        .bind(book.title)
        .bind(&book.authors)
        .bind(&book.publishers)
        .bind(book.date_published)
        .bind(book.abstract_text)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Book::from(row))
    }

    async fn delete(&self, id: i64) -> Result<(), String> {
        sqlx::query(
            r#"
            DELETE FROM books
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Book>, String> {
        let rows = sqlx::query_as::<_, BookRow>(
            r#"
            SELECT id, title, authors, publishers, date_published, abstract_text
            FROM books
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(Book::from).collect())
    }
}
