use super::models::Video;
use crate::storage::PostgresDatabase;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait VideosRepository: Send + Sync {
    async fn insert(&self, item: Video) -> Result<String, sqlx::Error>;
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
    async fn insert(&self, item: Video) -> Result<String, sqlx::Error> {
        let created_id: uuid::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO videos (title, description, categories, visibility) 
            VALUES ($1, $2, $3::category_enum[], $4::visibility_enum[]) 
            RETURNING id
            "#,
        )
        .bind(&item.title)
        .bind(&item.description)
        .bind(&item.categories)
        .bind(&vec![item.visibility])
        .fetch_one(self.db.get_conn())
        .await?;

        Ok(created_id.to_string())
    }
}
