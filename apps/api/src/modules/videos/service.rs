use super::models::{CreateVideoRequest, Video, VideoResponse};
use super::repository::VideosRepository;
use std::sync::Arc;

pub struct VideosService {
    repository: Arc<dyn VideosRepository>,
}

impl VideosService {
    pub fn new(repository: Arc<dyn VideosRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, req: CreateVideoRequest) -> Result<VideoResponse, String> {
        let item = Video::new(None, req.name);
        let created = self.repository.create(item).await?;
        Ok(VideoResponse::from(created))
    }

    pub async fn list(&self) -> Result<Vec<VideoResponse>, String> {
        let items = self.repository.list().await?;
        Ok(items.into_iter().map(VideoResponse::from).collect())
    }
}
