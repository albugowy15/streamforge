use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres pool");

    // Initialize database table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id BIGSERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            authors TEXT[] NOT NULL,
            publishers TEXT[] NOT NULL,
            date_published DATE NOT NULL,
            abstract_text TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize database table");

    pool
}
