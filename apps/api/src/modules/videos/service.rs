use crate::error::AppError;
use crate::modules::videos::models::{
    AbortVideoUploadResponse, CompleteVideoPartRequest, CompleteVideoUploadRequest,
    CompleteVideoUploadResponse, CreateVideoResponse, UploadVideoPartResponse,
    UploadVideoStatusResponse, UploadedVideoPart,
};
use aws_sdk_s3::types::CompletedPart;

use super::models::CreateVideoRequest;
use super::repository::VideosRepository;
use std::sync::Arc;
use tracing::{debug, info, warn};

const RECOMMENDED_PART_SIZE_BYTES: u64 = 8 * 1024 * 1024;

pub struct VideosService {
    repository: Arc<dyn VideosRepository>,
}

impl VideosService {
    pub fn new(repository: Arc<dyn VideosRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, req: CreateVideoRequest) -> Result<CreateVideoResponse, AppError> {
        let content_type = req.content_type.map(|value| value.to_string());
        info!(
            title = %req.title,
            categories = req.categories.len(),
            content_type = content_type.as_deref().unwrap_or("unknown"),
            "creating video metadata and starting resumable upload"
        );

        let video_id = self.repository.insert(req.into()).await?;
        let object_key = Self::source_object_key(&video_id);
        let upload = self
            .repository
            .initiate_multipart_upload(&object_key, content_type.as_deref())
            .await?;

        info!(
            video_id = %video_id,
            upload_id = %upload.upload_id,
            bucket = %upload.bucket,
            object_key = %upload.object_key,
            recommended_part_size_bytes = RECOMMENDED_PART_SIZE_BYTES,
            "resumable upload session started"
        );

        Ok(CreateVideoResponse {
            video_id,
            upload_id: upload.upload_id,
            recommended_part_size_bytes: RECOMMENDED_PART_SIZE_BYTES,
        })
    }

    pub async fn upload_part(
        &self,
        video_id: String,
        upload_id: String,
        part_number: i32,
        bytes: Vec<u8>,
    ) -> Result<UploadVideoPartResponse, AppError> {
        Self::validate_upload_id(&upload_id)?;
        Self::validate_part_number(part_number)?;

        if bytes.is_empty() {
            warn!(
                video_id = %video_id,
                upload_id = %upload_id,
                part_number,
                "rejecting empty video upload part"
            );
            return Err(AppError::BadRequest(
                "video part body cannot be empty".to_string(),
            ));
        }

        let size_bytes = bytes.len();
        let object_key = Self::source_object_key(&video_id);
        debug!(
            video_id = %video_id,
            upload_id = %upload_id,
            object_key = %object_key,
            part_number,
            size_bytes,
            "uploading video part"
        );

        let etag = self
            .repository
            .upload_part(&object_key, &upload_id, part_number, bytes)
            .await?;

        info!(
            video_id = %video_id,
            upload_id = %upload_id,
            part_number,
            size_bytes,
            etag = %etag,
            "video part uploaded"
        );

        Ok(UploadVideoPartResponse {
            video_id,
            upload_id,
            part_number,
            etag,
            size_bytes,
        })
    }

    pub async fn upload_status(
        &self,
        video_id: String,
        upload_id: String,
    ) -> Result<UploadVideoStatusResponse, AppError> {
        Self::validate_upload_id(&upload_id)?;

        let object_key = Self::source_object_key(&video_id);
        debug!(
            video_id = %video_id,
            upload_id = %upload_id,
            object_key = %object_key,
            "fetching upload status"
        );

        let uploaded_parts = self
            .repository
            .list_uploaded_parts(&object_key, &upload_id)
            .await?;

        let status = Self::build_upload_status(video_id, upload_id, uploaded_parts);

        info!(
            video_id = %status.video_id,
            upload_id = %status.upload_id,
            uploaded_parts = status.uploaded_parts.len(),
            uploaded_bytes = status.uploaded_bytes,
            next_part_number = status.next_part_number,
            "upload status fetched"
        );

        Ok(status)
    }

    pub async fn complete_upload(
        &self,
        video_id: String,
        req: CompleteVideoUploadRequest,
    ) -> Result<CompleteVideoUploadResponse, AppError> {
        Self::validate_upload_id(&req.upload_id)?;

        let object_key = Self::source_object_key(&video_id);
        info!(
            video_id = %video_id,
            upload_id = %req.upload_id,
            object_key = %object_key,
            request_includes_parts = req.parts.is_some(),
            "completing video upload"
        );

        let parts = match req.parts {
            Some(parts) => Self::complete_parts_from_request(parts)?,
            None => {
                debug!(
                    video_id = %video_id,
                    upload_id = %req.upload_id,
                    "completion request omitted parts; listing uploaded parts from storage"
                );
                let uploaded_parts = self
                    .repository
                    .list_uploaded_parts(&object_key, &req.upload_id)
                    .await?;
                Self::complete_parts_from_uploaded(uploaded_parts)?
            }
        };
        let part_count = parts.len();

        let completed = self
            .repository
            .complete_multipart_upload(&object_key, &req.upload_id, parts)
            .await?;

        info!(
            video_id = %video_id,
            upload_id = %req.upload_id,
            bucket = %completed.bucket,
            object_key = %completed.object_key,
            part_count,
            "video upload completed"
        );

        Ok(CompleteVideoUploadResponse {
            video_id,
            upload_id: req.upload_id,
        })
    }

    pub async fn abort_upload(
        &self,
        video_id: String,
        upload_id: String,
    ) -> Result<AbortVideoUploadResponse, AppError> {
        Self::validate_upload_id(&upload_id)?;

        let object_key = Self::source_object_key(&video_id);
        warn!(
            video_id = %video_id,
            upload_id = %upload_id,
            object_key = %object_key,
            "aborting video upload"
        );

        self.repository
            .abort_multipart_upload(&object_key, &upload_id)
            .await?;

        info!(
            video_id = %video_id,
            upload_id = %upload_id,
            object_key = %object_key,
            "video upload aborted"
        );

        Ok(AbortVideoUploadResponse {
            video_id,
            upload_id,
            aborted: true,
        })
    }

    fn source_object_key(video_id: &str) -> String {
        format!("videos/{video_id}/source")
    }

    fn validate_upload_id(upload_id: &str) -> Result<(), AppError> {
        if upload_id.trim().is_empty() {
            warn!("rejecting upload request without upload_id");
            return Err(AppError::BadRequest("upload_id is required".to_string()));
        }
        Ok(())
    }

    fn validate_part_number(part_number: i32) -> Result<(), AppError> {
        if !(1..=10_000).contains(&part_number) {
            warn!(part_number, "rejecting invalid video upload part number");
            return Err(AppError::BadRequest(
                "part_number must be between 1 and 10000".to_string(),
            ));
        }
        Ok(())
    }

    fn build_upload_status(
        video_id: String,
        upload_id: String,
        uploaded_parts: Vec<UploadedVideoPart>,
    ) -> UploadVideoStatusResponse {
        let uploaded_bytes = uploaded_parts.iter().map(|part| part.size_bytes).sum();
        let next_part_number = uploaded_parts
            .iter()
            .map(|part| part.part_number)
            .max()
            .unwrap_or(0)
            + 1;

        UploadVideoStatusResponse {
            video_id,
            upload_id,
            uploaded_parts,
            uploaded_bytes,
            next_part_number,
        }
    }

    fn complete_parts_from_request(
        parts: Vec<CompleteVideoPartRequest>,
    ) -> Result<Vec<CompletedPart>, AppError> {
        if parts.is_empty() {
            warn!("rejecting complete upload request without parts");
            return Err(AppError::BadRequest(
                "at least one uploaded part is required to complete an upload".to_string(),
            ));
        }

        let mut parts = parts;
        parts.sort_by_key(|part| part.part_number);
        parts
            .into_iter()
            .map(|part| {
                Self::validate_part_number(part.part_number)?;
                if part.etag.trim().is_empty() {
                    warn!(
                        part_number = part.part_number,
                        "rejecting complete upload request with empty part etag"
                    );
                    return Err(AppError::BadRequest("part etag is required".to_string()));
                }
                Ok(CompletedPart::builder()
                    .part_number(part.part_number)
                    .e_tag(part.etag)
                    .build())
            })
            .collect()
    }

    fn complete_parts_from_uploaded(
        parts: Vec<UploadedVideoPart>,
    ) -> Result<Vec<CompletedPart>, AppError> {
        if parts.is_empty() {
            warn!("cannot complete upload because storage returned no uploaded parts");
            return Err(AppError::BadRequest(
                "no uploaded parts found for upload_id".to_string(),
            ));
        }

        let parts = parts
            .into_iter()
            .map(|part| CompleteVideoPartRequest {
                part_number: part.part_number,
                etag: part.etag,
            })
            .collect();

        Self::complete_parts_from_request(parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::videos::models::{
        CompletedVideoUpload, InitiatedVideoUpload, Video, VideoContentType, Visibility,
    };
    use async_trait::async_trait;

    struct MockVideosRepository;

    #[async_trait]
    impl VideosRepository for MockVideosRepository {
        async fn insert(&self, _item: Video) -> Result<String, sqlx::Error> {
            Ok("test-uuid-123".to_string())
        }

        async fn initiate_multipart_upload(
            &self,
            object_key: &str,
            _content_type: Option<&str>,
        ) -> Result<InitiatedVideoUpload, AppError> {
            Ok(InitiatedVideoUpload {
                upload_id: "upload-123".to_string(),
                bucket: "streamforge".to_string(),
                object_key: object_key.to_string(),
            })
        }

        async fn upload_part(
            &self,
            _object_key: &str,
            _upload_id: &str,
            _part_number: i32,
            _bytes: Vec<u8>,
        ) -> Result<String, AppError> {
            Ok("etag-1".to_string())
        }

        async fn list_uploaded_parts(
            &self,
            _object_key: &str,
            _upload_id: &str,
        ) -> Result<Vec<UploadedVideoPart>, AppError> {
            Ok(vec![
                UploadedVideoPart {
                    part_number: 1,
                    etag: "etag-1".to_string(),
                    size_bytes: 10,
                },
                UploadedVideoPart {
                    part_number: 2,
                    etag: "etag-2".to_string(),
                    size_bytes: 20,
                },
            ])
        }

        async fn complete_multipart_upload(
            &self,
            object_key: &str,
            _upload_id: &str,
            _parts: Vec<CompletedPart>,
        ) -> Result<CompletedVideoUpload, AppError> {
            Ok(CompletedVideoUpload {
                bucket: "streamforge".to_string(),
                object_key: object_key.to_string(),
                etag: Some("complete-etag".to_string()),
            })
        }

        async fn abort_multipart_upload(
            &self,
            _object_key: &str,
            _upload_id: &str,
        ) -> Result<(), AppError> {
            Ok(())
        }
    }

    struct MockFailVideosRepository;

    #[async_trait]
    impl VideosRepository for MockFailVideosRepository {
        async fn insert(&self, _item: Video) -> Result<String, sqlx::Error> {
            Err(sqlx::Error::RowNotFound)
        }

        async fn initiate_multipart_upload(
            &self,
            _object_key: &str,
            _content_type: Option<&str>,
        ) -> Result<InitiatedVideoUpload, AppError> {
            unreachable!()
        }

        async fn upload_part(
            &self,
            _object_key: &str,
            _upload_id: &str,
            _part_number: i32,
            _bytes: Vec<u8>,
        ) -> Result<String, AppError> {
            unreachable!()
        }

        async fn list_uploaded_parts(
            &self,
            _object_key: &str,
            _upload_id: &str,
        ) -> Result<Vec<UploadedVideoPart>, AppError> {
            unreachable!()
        }

        async fn complete_multipart_upload(
            &self,
            _object_key: &str,
            _upload_id: &str,
            _parts: Vec<CompletedPart>,
        ) -> Result<CompletedVideoUpload, AppError> {
            unreachable!()
        }

        async fn abort_multipart_upload(
            &self,
            _object_key: &str,
            _upload_id: &str,
        ) -> Result<(), AppError> {
            unreachable!()
        }
    }

    fn valid_create_request() -> CreateVideoRequest {
        CreateVideoRequest {
            title: "A very long video title that meets the validation requirements".to_string(),
            description: "An even longer description that also meets the validation requirements for this test case specifically.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
            file_name: Some("sample.mp4".to_string()),
            content_type: Some(VideoContentType::Mp4),
        }
    }

    #[tokio::test]
    async fn test_create_video_success() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        let res = service.create(valid_create_request()).await.unwrap();
        assert_eq!(res.video_id, "test-uuid-123");
        assert_eq!(res.upload_id, "upload-123");
        assert_eq!(res.recommended_part_size_bytes, 8 * 1024 * 1024);
    }

    #[tokio::test]
    async fn test_create_video_repository_error() {
        let repo = Arc::new(MockFailVideosRepository);
        let service = VideosService::new(repo);

        let res = service.create(valid_create_request()).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            AppError::Internal(msg) => assert!(msg.contains("no rows returned")),
            _ => panic!("Expected Internal error"),
        }
    }

    #[tokio::test]
    async fn test_upload_part_success() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        let res = service
            .upload_part(
                "video-1".to_string(),
                "upload-123".to_string(),
                1,
                vec![1, 2, 3],
            )
            .await
            .unwrap();

        assert_eq!(res.video_id, "video-1");
        assert_eq!(res.part_number, 1);
        assert_eq!(res.etag, "etag-1");
        assert_eq!(res.size_bytes, 3);
    }

    #[tokio::test]
    async fn test_upload_part_rejects_invalid_input() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        let invalid_part = service
            .upload_part("video-1".to_string(), "upload-123".to_string(), 0, vec![1])
            .await;
        assert!(matches!(invalid_part, Err(AppError::BadRequest(_))));

        let empty_body = service
            .upload_part("video-1".to_string(), "upload-123".to_string(), 1, vec![])
            .await;
        assert!(matches!(empty_body, Err(AppError::BadRequest(_))));
    }

    #[tokio::test]
    async fn test_upload_status_calculates_progress() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        let res = service
            .upload_status("video-1".to_string(), "upload-123".to_string())
            .await
            .unwrap();

        assert_eq!(res.uploaded_parts.len(), 2);
        assert_eq!(res.uploaded_bytes, 30);
        assert_eq!(res.next_part_number, 3);
    }

    #[tokio::test]
    async fn test_complete_upload_uses_listed_parts_when_request_parts_are_absent() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        service
            .complete_upload(
                "video-1".to_string(),
                CompleteVideoUploadRequest {
                    upload_id: "upload-123".to_string(),
                    parts: None,
                },
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_abort_upload_success() {
        let repo = Arc::new(MockVideosRepository);
        let service = VideosService::new(repo);

        let res = service
            .abort_upload("video-1".to_string(), "upload-123".to_string())
            .await
            .unwrap();

        assert!(res.aborted);
    }
}
