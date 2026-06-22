pub mod config;
pub mod error;
pub mod form;
pub mod json;
pub mod modules;
pub mod query;
pub mod state;
pub mod storage;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{Method, StatusCode, header},
    response::IntoResponse,
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
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    error::AppError,
    modules::videos::{VideoUploadRepository, VideosRouter, VideosService},
    state::AppState,
    storage::{PostgresDatabase, S3},
};

pub fn build_app(db: Arc<PostgresDatabase>, s3: Arc<S3>) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "StreamForge API", description = "StreamForge management API")
        )
    )]
    struct ApiDoc;

    let videos_repository = Arc::new(VideoUploadRepository::new(db.clone(), s3.clone()));
    let videos_service = VideosService::new(videos_repository);

    let app_state = Arc::new(AppState { videos_service });

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(utoipa_axum::routes!(check_health))
        .nest("/api/v1/", VideosRouter::new())
        .split_for_parts();

    let router = router.with_state(app_state);

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .layer(DefaultBodyLimit::max(64 * 1024 * 1024))
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
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(300)),
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

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Check health")
    ),
    tag = "Health"
)]
async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}
