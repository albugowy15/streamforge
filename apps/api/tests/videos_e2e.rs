use sqlx::Connection;
use sqlx::migrate::Migrator;
use sqlx::{Executor, PgConnection, Row};
use std::path::Path;
use std::sync::{Arc, OnceLock};
use streamforge_api::config::Config;
use streamforge_api::storage::{PostgresDatabase, S3};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db: Arc<PostgresDatabase>,
}

/// GLOBAL INFRASTRUCTURE STRATEGY:
///
/// Starting Docker containers is expensive (1-2 seconds per container).
/// To keep tests fast, we use a "Shared Container + Isolated Database" approach:
///
/// 1. Shared Containers: Postgres and rustfs are started once per test run using `OnceLock`.
/// 2. Isolated Databases: Each call to `spawn_app` creates a brand-new database within the
///    shared Postgres instance.
/// 3. Automatic Cleanup: Testcontainers handles stopping the containers when the test process exits.
///
/// This provides the speed of shared infrastructure with the perfect isolation of fresh containers.

// OnceLock is a thread-safe synchronization primitive that can be written to only once.
// We use it here to ensure that the heavy Docker containers are initialized lazily
// and only a single time, even when multiple tests run in parallel.
static POSTGRES_CONTAINER: OnceLock<ContainerAsync<GenericImage>> = OnceLock::new();
static RUSTFS_CONTAINER: OnceLock<ContainerAsync<GenericImage>> = OnceLock::new();

/// Returns a reference to the global Postgres container, starting it if necessary.
async fn get_postgres_container() -> &'static ContainerAsync<GenericImage> {
    if let Some(c) = POSTGRES_CONTAINER.get() {
        return c;
    }

    use testcontainers::core::{IntoContainerPort, WaitFor};
    let postgres_image = GenericImage::new("postgres", "18-alpine")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_exposed_port(5432.tcp())
        .with_env_var("POSTGRES_DB", "postgres")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres");

    let container = postgres_image.start().await.unwrap();
    // set() might fail if another test started the container at the exact same microsecond,
    // which is fine as we just want one instance.
    POSTGRES_CONTAINER.set(container).ok();
    POSTGRES_CONTAINER.get().unwrap()
}

/// Returns a reference to the global rustfs container, starting it if necessary.
async fn get_rustfs_container() -> &'static ContainerAsync<GenericImage> {
    if let Some(c) = RUSTFS_CONTAINER.get() {
        return c;
    }

    use testcontainers::core::{IntoContainerPort, WaitFor};
    let rustfs_image = GenericImage::new("rustfs/rustfs", "latest")
        .with_wait_for(WaitFor::message_on_stdout("Starting: /usr/bin/rustfs"))
        .with_exposed_port(9000.tcp())
        .with_env_var("RUSTFS_ACCESS_KEY", "rustfsadmin")
        .with_env_var("RUSTFS_SECRET_KEY", "rustfsadmin")
        .with_env_var("RUSTFS_ADDRESS", "0.0.0.0:9000");

    let container = rustfs_image.start().await.unwrap();
    RUSTFS_CONTAINER.set(container).ok();
    RUSTFS_CONTAINER.get().unwrap()
}

/// Bootstraps a fresh application instance for testing.
///
/// This involves:
/// 1. Connecting to the shared global containers.
/// 2. Creating a unique, isolated database for this specific test.
/// 3. Running migrations on the new database.
/// 4. Starting the API server on a random available port.
pub async fn spawn_app() -> TestApp {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .try_init();

    // 1. Get shared containers
    let pg_container = get_postgres_container().await;
    let fs_container = get_rustfs_container().await;

    let pg_host = pg_container.get_host().await.unwrap();
    let pg_port = pg_container.get_host_port_ipv4(5432).await.unwrap();

    let fs_host = fs_container.get_host().await.unwrap();
    let fs_port = fs_container.get_host_port_ipv4(9000).await.unwrap();

    // 2. Create a unique database for THIS test for isolation
    let db_name = format!("db_{}", Uuid::new_v4().simple());
    let maintenance_url = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        pg_host, pg_port
    );
    let mut conn = PgConnection::connect(&maintenance_url).await.unwrap();
    conn.execute(format!(r#"CREATE DATABASE "{}""#, db_name).as_str())
        .await
        .unwrap();

    let database_url = format!(
        "postgres://postgres:postgres@{}:{}/{}",
        pg_host, pg_port, db_name
    );

    // 3. Setup Config
    let config = Config {
        database_url,
        region: "us-east-1".to_string(),
        access_key_id: "rustfsadmin".to_string(),
        secret_access_key: "rustfsadmin".to_string(),
        endpoint_url: format!("http://{}:{}", fs_host, fs_port),
    };

    // 4. Initialize Database and Migrations
    let db = Arc::new(PostgresDatabase::new(&config).await.unwrap());
    let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
    migrator.run(db.get_conn()).await.unwrap();

    // 5. Initialize Services and App
    let s3 = Arc::new(S3::new(&config).await);
    let app = streamforge_api::build_app(db.clone(), s3);

    // 6. Start Server on random port
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestApp { address, db }
}

#[tokio::test]
async fn test_post_videos_e2e() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // 1. Success case
    let payload = serde_json::json!({
        "title": "A very long video title that meets the validation requirements",
        "description": "An even longer description that also meets the validation requirements for this test case specifically and it has more than one hundred characters.",
        "visibility": "public",
        "categories": ["action", "comedy"]
    });

    let response = client
        .post(&format!("{}/api/v1/videos", app.address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request.");

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap();
        panic!("Request failed with status {}: {}", status, body);
    }
    let body: serde_json::Value = response.json().await.unwrap();
    let video_id = body["data"]["video_id"]
        .as_str()
        .expect("video_id not found in response");

    let row = sqlx::query("SELECT title, description FROM videos WHERE id = $1")
        .bind(uuid::Uuid::parse_str(video_id).unwrap())
        .fetch_one(app.db.get_conn())
        .await
        .expect("Failed to fetch video from database");

    let title: String = row.get("title");
    let description: String = row.get("description");

    assert_eq!(
        title,
        "A very long video title that meets the validation requirements"
    );
    assert_eq!(
        description,
        "An even longer description that also meets the validation requirements for this test case specifically and it has more than one hundred characters."
    );

    // 2. Invalid JSON case
    let response = client
        .post(&format!("{}/api/v1/videos", app.address))
        .header("Content-Type", "application/json")
        .body("invalid-json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    // 3. Validation error case
    let payload = serde_json::json!({
        "title": "Short",
        "description": "Short",
        "visibility": "public",
        "categories": ["action"]
    });

    let response = client
        .post(&format!("{}/api/v1/videos", app.address))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["errors"].is_object());
}
