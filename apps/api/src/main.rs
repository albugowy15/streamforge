mod config;
mod modules;
mod shared;
mod storage;

use axum::Router;
use axum::http::HeaderValue;
use axum::http::Method;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;
use crate::modules::books::BookRouter;
use crate::modules::books::BookService;
use crate::modules::books::PostgresBookRepository;
use crate::shared::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Config::from_env()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("start database connection");
    let db = Arc::new(storage::PostgresDatabase::new(&config).await?);

    tracing::debug!("start s3 connection");
    let s3 = Arc::new(storage::S3::new(&config).await);
    let total_buckets = s3
        .get_client()
        .list_buckets()
        .send()
        .await?
        .buckets
        .iter()
        .len();
    tracing::debug!("found {:?} buckets", total_buckets);

    tracing::debug!("start database migrations");
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(db.get_conn()).await?;

    let book_repository = Arc::new(PostgresBookRepository::new(db.clone(), s3.clone()));
    let book_service = BookService::new(book_repository);

    let app_state = Arc::new(AppState { book_service });

    let app = Router::new()
        .route("/health", get(check_health))
        .merge(BookRouter::new())
        .with_state(app_state)
        .layer((
            TraceLayer::new_for_http(),
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
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
        .fallback(not_found_handler);

    let listener = TcpListener::bind("0.0.0.0:5000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see her")
}

async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
