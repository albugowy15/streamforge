use super::models::Video;
use crate::storage::PostgresDatabase;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait VideosRepository: Send + Sync {
    async fn create(&self, item: Video) -> Result<Video, String>;
    async fn list(&self) -> Result<Vec<Video>, String>;
}

pub struct PostgresVideosRepository {
    db: Arc<PostgresDatabase>,
}

impl PostgresVideosRepository {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl VideosRepository for PostgresVideosRepository {
    async fn create(&self, item: Video) -> Result<Video, String> {
        // Placeholder implementation
        let mut created = item;
        created.id = Some(1);
        Ok(created)
    }

    async fn list(&self) -> Result<Vec<Video>, String> {
        // Placeholder implementation
        Ok(vec![Video::new(Some(1), "Scaffolded Item".to_string())])
    }
}
