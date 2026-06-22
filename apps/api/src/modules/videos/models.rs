use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use crate::json::{AppJson, JsonData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub enum VideoContentType {
    #[serde(rename = "video/mp4")]
    Mp4,
    #[serde(rename = "video/webm")]
    Webm,
    #[serde(rename = "video/ogg")]
    Ogg,
    #[serde(rename = "video/quicktime")]
    Quicktime,
    #[serde(rename = "video/x-msvideo")]
    Avi,
    #[serde(rename = "video/x-matroska")]
    Matroska,
    #[serde(rename = "video/mp2t")]
    MpegTs,
}

impl Display for VideoContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content_type = match self {
            Self::Mp4 => "video/mp4",
            Self::Webm => "video/webm",
            Self::Ogg => "video/ogg",
            Self::Quicktime => "video/quicktime",
            Self::Avi => "video/x-msvideo",
            Self::Matroska => "video/x-matroska",
            Self::MpegTs => "video/mp2t",
        };

        f.write_str(content_type)
    }
}

fn validate_trimmed_non_empty(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("required"));
    }

    Ok(())
}

// begin REQUEST DTOS
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateVideoRequest {
    #[validate(length(min = 20, max = 200))]
    pub title: String,
    #[validate(length(min = 100, max = 400))]
    pub description: String,
    pub visibility: Visibility,
    #[validate(length(min = 2, max = 10))]
    pub categories: Vec<String>,
    #[validate(length(max = 255))]
    pub file_name: Option<String>,
    pub content_type: Option<VideoContentType>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, ToSchema, Validate)]
pub struct CompleteVideoPartRequest {
    #[validate(range(min = 1, max = 10_000))]
    pub part_number: i32,
    #[validate(
        length(min = 1, max = 256),
        custom(function = "validate_trimmed_non_empty")
    )]
    pub etag: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CompleteVideoUploadRequest {
    #[validate(
        length(min = 1, max = 255),
        custom(function = "validate_trimmed_non_empty")
    )]
    pub upload_id: String,
    #[validate(nested)]
    #[validate(length(min = 1, max = 10_000))]
    pub parts: Option<Vec<CompleteVideoPartRequest>>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UploadVideoQuery {
    #[validate(
        length(min = 1, max = 255),
        custom(function = "validate_trimmed_non_empty")
    )]
    pub upload_id: String,
}
// end REQUEST DTOS

// begin RESPONSE DTOS
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateVideoResponse {
    pub video_id: String,
    pub upload_id: String,
    pub recommended_part_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, ToSchema)]
pub struct UploadedVideoPart {
    pub part_number: i32,
    pub etag: String,
    pub size_bytes: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadVideoPartResponse {
    pub video_id: String,
    pub upload_id: String,
    pub part_number: i32,
    pub etag: String,
    pub size_bytes: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadVideoStatusResponse {
    pub video_id: String,
    pub upload_id: String,
    pub uploaded_parts: Vec<UploadedVideoPart>,
    pub uploaded_bytes: i64,
    pub next_part_number: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompleteVideoUploadResponse {
    pub video_id: String,
    pub upload_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AbortVideoUploadResponse {
    pub video_id: String,
    pub upload_id: String,
    pub aborted: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateVideoResponseBody {
    pub data: CreateVideoResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadVideoPartResponseBody {
    pub data: UploadVideoPartResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadVideoStatusResponseBody {
    pub data: UploadVideoStatusResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompleteVideoUploadResponseBody {
    pub data: CompleteVideoUploadResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AbortVideoUploadResponseBody {
    pub data: AbortVideoUploadResponse,
}

pub type CreateVideoResponseJson = AppJson<JsonData<CreateVideoResponse>>;
pub type UploadVideoPartResponseJson = AppJson<JsonData<UploadVideoPartResponse>>;
pub type UploadVideoStatusResponseJson = AppJson<JsonData<UploadVideoStatusResponse>>;
pub type CompleteVideoUploadResponseJson = AppJson<JsonData<CompleteVideoUploadResponse>>;
pub type AbortVideoUploadResponseJson = AppJson<JsonData<AbortVideoUploadResponse>>;

impl From<CreateVideoResponse> for JsonData<CreateVideoResponse> {
    fn from(value: CreateVideoResponse) -> Self {
        JsonData { data: value }
    }
}

impl From<UploadVideoPartResponse> for JsonData<UploadVideoPartResponse> {
    fn from(value: UploadVideoPartResponse) -> Self {
        JsonData { data: value }
    }
}

impl From<UploadVideoStatusResponse> for JsonData<UploadVideoStatusResponse> {
    fn from(value: UploadVideoStatusResponse) -> Self {
        JsonData { data: value }
    }
}

impl From<CompleteVideoUploadResponse> for JsonData<CompleteVideoUploadResponse> {
    fn from(value: CompleteVideoUploadResponse) -> Self {
        JsonData { data: value }
    }
}

impl From<AbortVideoUploadResponse> for JsonData<AbortVideoUploadResponse> {
    fn from(value: AbortVideoUploadResponse) -> Self {
        JsonData { data: value }
    }
}
// end RESPONSE DTOS

// begin INTERNAL DTOS
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiatedVideoUpload {
    pub upload_id: String,
    pub bucket: String,
    pub object_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedVideoUpload {
    pub bucket: String,
    pub object_key: String,
    pub etag: Option<String>,
}
// end INTERNAL DTOS

// begin TABLE
#[derive(Default)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub visibility: Visibility,
    pub categories: Vec<String>,
}
#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "visibility_enum", rename_all = "lowercase")]
pub enum Visibility {
    Private,
    #[default]
    Public,
}

impl From<CreateVideoRequest> for Video {
    fn from(value: CreateVideoRequest) -> Self {
        Self {
            title: value.title,
            description: value.description,
            visibility: value.visibility,
            categories: value.categories,
            ..Default::default()
        }
    }
}
// end TABLE

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_create_video_request_validation() {
        // Valid request
        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
            file_name: Some("sample-video.mp4".to_string()),
            content_type: Some(VideoContentType::Mp4),
        };
        assert!(request.validate().is_ok());

        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
            file_name: Some("sample-video.mov".to_string()),
            content_type: Some(VideoContentType::Quicktime),
        };
        assert!(request.validate().is_ok());

        // Invalid title (too short)
        let request = CreateVideoRequest {
            title: "Short title".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
            file_name: None,
            content_type: None,
        };
        assert!(request.validate().is_err());

        // Invalid description (too short)
        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "Too short".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
            file_name: None,
            content_type: None,
        };
        assert!(request.validate().is_err());

        // Invalid categories (too few)
        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string()],
            file_name: None,
            content_type: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_video_request_rejects_invalid_content_type() {
        let payload = json!({
            "title": "A valid video title that is long enough",
            "description": "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.",
            "visibility": "public",
            "categories": ["action", "comedy"],
            "file_name": "sample-video.png",
            "content_type": "image/png"
        });

        let request = serde_json::from_value::<CreateVideoRequest>(payload);
        assert!(request.is_err());
    }

    #[test]
    fn test_complete_upload_request_validation() {
        let request = CompleteVideoUploadRequest {
            upload_id: "upload-123".to_string(),
            parts: Some(vec![CompleteVideoPartRequest {
                part_number: 1,
                etag: "etag-1".to_string(),
            }]),
        };
        assert!(request.validate().is_ok());

        let request = CompleteVideoUploadRequest {
            upload_id: String::new(),
            parts: None,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_complete_upload_request_rejects_invalid_parts() {
        let request = CompleteVideoUploadRequest {
            upload_id: "upload-123".to_string(),
            parts: Some(vec![CompleteVideoPartRequest {
                part_number: 0,
                etag: "   ".to_string(),
            }]),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_upload_video_query_validation() {
        let query = UploadVideoQuery {
            upload_id: "upload-123".to_string(),
        };
        assert!(query.validate().is_ok());

        let query = UploadVideoQuery {
            upload_id: "   ".to_string(),
        };
        assert!(query.validate().is_err());
    }
}
