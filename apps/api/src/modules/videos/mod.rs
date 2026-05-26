pub mod controller;
pub mod models;
pub mod repository;
mod router;
pub mod service;

pub use {
    models::Video, repository::PostgresVideosRepository, repository::VideosRepository,
    router::VideosRouter, service::VideosService,
};
