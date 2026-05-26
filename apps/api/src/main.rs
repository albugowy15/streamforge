use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use std::path::Path;
use std::sync::Arc;
use streamforge_api::config::Config;
use streamforge_api::storage;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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

    let app = streamforge_api::build_app(db, s3);

    let listener = TcpListener::bind("0.0.0.0:5000").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
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
