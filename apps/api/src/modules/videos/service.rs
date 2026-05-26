use crate::error::AppError;
use crate::modules::videos::models::CreateVideoResponse;

use super::models::CreateVideoRequest;
use super::repository::VideosRepository;
use std::sync::Arc;

pub struct VideosService {
    repository: Arc<dyn VideosRepository>,
}

impl VideosService {
    pub fn new(repository: Arc<dyn VideosRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, req: CreateVideoRequest) -> Result<CreateVideoResponse, AppError> {
        Ok(CreateVideoResponse {
            video_id: self.repository.insert(req.into()).await?,
            upload_id: String::from("upload_id"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::videos::models::Video;
    use async_trait::async_trait;

    struct MockVideosRepository;

    #[async_trait]
    impl VideosRepository for MockVideosRepository {
        async fn insert(&self, _item: Video) -> Result<String, sqlx::Error> {
            Ok("test-uuid-123".to_string())
        }
    }

    struct MockFailVideosRepository;

    #[async_trait]
    impl VideosRepository for MockFailVideosRepository {
        async fn insert(&self, _item: Video) -> Result<String, sqlx::Error> {
            Err(sqlx::Error::RowNotFound)
        }
    }

    #[tokio::test]
    async fn test_create_video_success() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        let req = CreateVideoRequest {
            title: "A very long video title that meets the validation requirements".to_string(),
            description: "An even longer description that also meets the validation requirements for this test case specifically.".to_string(),
            visibility: crate::modules::videos::models::Visibility::Public,
            categories: vec!["action".to_string()],
        };

        let res = service.create(req).await.unwrap();
        assert_eq!(res.video_id, "test-uuid-123");
    }

    #[tokio::test]
    async fn test_create_video_repository_error() {
        let repo = Arc::new(MockFailVideosRepository);
        let service = VideosService::new(repo);

        let req = CreateVideoRequest {
            title: "A very long video title that meets the validation requirements".to_string(),
            description: "An even longer description that also meets the validation requirements for this test case specifically.".to_string(),
            visibility: crate::modules::videos::models::Visibility::Public,
            categories: vec!["action".to_string()],
        };

        let res = service.create(req).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            AppError::Internal(msg) => assert!(msg.contains("no rows returned")),
            _ => panic!("Expected Internal error"),
        }
    }
}
