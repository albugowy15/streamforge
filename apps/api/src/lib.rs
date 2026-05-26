pub mod config;
pub mod error;
pub mod form;
pub mod json;
pub mod modules;
pub mod state;
pub mod storage;

use axum::{
    Router,
    http::{Method, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::RequestDecompressionLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    error::AppError,
    modules::{
        books::{BookRouter, BookService, PostgresBookRepository},
        videos::{PostgresVideosRepository, VideosRouter, VideosService},
    },
    state::AppState,
    storage::{PostgresDatabase, S3},
};

pub fn build_app(db: Arc<PostgresDatabase>, s3: Arc<S3>) -> Router {
    let book_repository = Arc::new(PostgresBookRepository::new(db.clone(), s3.clone()));
    let videos_repository = Arc::new(PostgresVideosRepository::new(db.clone()));
    let book_service = BookService::new(book_repository);
    let videos_service = VideosService::new(videos_repository);

    let app_state = Arc::new(AppState {
        book_service,
        videos_service,
    });

    Router::new()
        .route("/health", get(check_health))
        .merge(BookRouter::new())
        .merge(VideosRouter::new())
        .with_state(app_state)
        .layer((
            TraceLayer::new_for_http(),
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ]),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ))
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .fallback(not_found_handler)
}

async fn not_found_handler() -> impl IntoResponse {
    AppError::NotFound
}

async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}
