use std::time::Duration;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::config::Config;

pub struct PostgresDatabase(Pool<Postgres>);

impl PostgresDatabase {
    pub async fn new(config: &Config) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new()
            .max_connections(50)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&config.database_url)
            .await?;

        Ok(Self(db))
    }

    pub fn get_conn(&self) -> &Pool<Postgres> {
        &self.0
    }
}
