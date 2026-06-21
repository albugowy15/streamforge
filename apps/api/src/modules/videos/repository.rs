use super::models::{CompletedVideoUpload, InitiatedVideoUpload, UploadedVideoPart, Video};
use crate::{
    error::AppError,
    storage::{PostgresDatabase, S3},
};
use async_trait::async_trait;
use aws_sdk_s3::{
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart},
};
use std::sync::Arc;

#[async_trait]
pub trait VideosRepository: Send + Sync {
    async fn insert(&self, item: Video) -> Result<String, sqlx::Error>;
    async fn initiate_multipart_upload(
        &self,
        object_key: &str,
        content_type: Option<&str>,
    ) -> Result<InitiatedVideoUpload, AppError>;
    async fn upload_part(
        &self,
        object_key: &str,
        upload_id: &str,
        part_number: i32,
        bytes: Vec<u8>,
    ) -> Result<String, AppError>;
    async fn list_uploaded_parts(
        &self,
        object_key: &str,
        upload_id: &str,
    ) -> Result<Vec<UploadedVideoPart>, AppError>;
    async fn complete_multipart_upload(
        &self,
        object_key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<CompletedVideoUpload, AppError>;
    async fn abort_multipart_upload(
        &self,
        object_key: &str,
        upload_id: &str,
    ) -> Result<(), AppError>;
}

pub struct VideoUploadRepository {
    db: Arc<PostgresDatabase>,
    s3: Arc<S3>,
}

impl VideoUploadRepository {
    pub fn new(db: Arc<PostgresDatabase>, s3: Arc<S3>) -> Self {
        Self { db, s3 }
    }

    fn map_storage_error(action: &str, err: impl std::fmt::Display) -> AppError {
        AppError::Internal(format!("failed to {}: {}", action, err))
    }
}

#[async_trait]
impl VideosRepository for VideoUploadRepository {
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
        .bind(vec![item.visibility])
        .fetch_one(self.db.get_conn())
        .await?;

        Ok(created_id.to_string())
    }

    async fn initiate_multipart_upload(
        &self,
        object_key: &str,
        content_type: Option<&str>,
    ) -> Result<InitiatedVideoUpload, AppError> {
        self.s3
            .ensure_bucket_exists()
            .await
            .map_err(|err| Self::map_storage_error("ensure S3 bucket exists", err))?;

        let mut request = self
            .s3
            .get_client()
            .create_multipart_upload()
            .bucket(self.s3.bucket())
            .key(object_key);

        if let Some(content_type) = content_type {
            request = request.content_type(content_type);
        }

        let output = request
            .send()
            .await
            .map_err(|err| Self::map_storage_error("create multipart upload", err))?;

        let upload_id = output
            .upload_id()
            .ok_or_else(|| AppError::Internal("S3 did not return an upload id".to_string()))?
            .to_string();

        Ok(InitiatedVideoUpload {
            upload_id,
            bucket: output.bucket().unwrap_or(self.s3.bucket()).to_string(),
            object_key: output.key().unwrap_or(object_key).to_string(),
        })
    }

    async fn upload_part(
        &self,
        object_key: &str,
        upload_id: &str,
        part_number: i32,
        bytes: Vec<u8>,
    ) -> Result<String, AppError> {
        let output = self
            .s3
            .get_client()
            .upload_part()
            .bucket(self.s3.bucket())
            .key(object_key)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map_err(|err| Self::map_storage_error("upload video part", err))?;

        output
            .e_tag()
            .map(str::to_string)
            .ok_or_else(|| AppError::Internal("S3 did not return an ETag".to_string()))
    }

    async fn list_uploaded_parts(
        &self,
        object_key: &str,
        upload_id: &str,
    ) -> Result<Vec<UploadedVideoPart>, AppError> {
        let output = self
            .s3
            .get_client()
            .list_parts()
            .bucket(self.s3.bucket())
            .key(object_key)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|err| Self::map_storage_error("list uploaded video parts", err))?;

        let mut parts = output
            .parts()
            .iter()
            .filter_map(|part| {
                Some(UploadedVideoPart {
                    part_number: part.part_number()?,
                    etag: part.e_tag()?.to_string(),
                    size_bytes: part.size().unwrap_or_default(),
                })
            })
            .collect::<Vec<_>>();
        parts.sort_by_key(|part| part.part_number);

        Ok(parts)
    }

    async fn complete_multipart_upload(
        &self,
        object_key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<CompletedVideoUpload, AppError> {
        let multipart_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();

        let output = self
            .s3
            .get_client()
            .complete_multipart_upload()
            .bucket(self.s3.bucket())
            .key(object_key)
            .upload_id(upload_id)
            .multipart_upload(multipart_upload)
            .send()
            .await
            .map_err(|err| Self::map_storage_error("complete multipart upload", err))?;

        Ok(CompletedVideoUpload {
            bucket: output.bucket().unwrap_or(self.s3.bucket()).to_string(),
            object_key: output.key().unwrap_or(object_key).to_string(),
            etag: output.e_tag().map(str::to_string),
        })
    }

    async fn abort_multipart_upload(
        &self,
        object_key: &str,
        upload_id: &str,
    ) -> Result<(), AppError> {
        self.s3
            .get_client()
            .abort_multipart_upload()
            .bucket(self.s3.bucket())
            .key(object_key)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|err| Self::map_storage_error("abort multipart upload", err))?;

        Ok(())
    }
}
