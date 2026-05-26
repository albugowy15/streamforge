pub mod controller;
pub mod models;
pub mod repository;
mod router;
pub mod service;

pub use {
    models::Book, repository::BookRepository, repository::PostgresBookRepository,
    router::BookRouter, service::BookService,
};
