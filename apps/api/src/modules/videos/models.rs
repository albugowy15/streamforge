use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::json::{AppJson, JsonData};

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
// end REQUEST DTOS

// begin RESPONSE DTOS
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateVideoResponse {
    pub video_id: String,
    pub upload_id: String,
}

pub type CreateVideoResponseJson = AppJson<JsonData<CreateVideoResponse>>;

impl From<CreateVideoResponse> for JsonData<CreateVideoResponse> {
    fn from(value: CreateVideoResponse) -> Self {
        JsonData { data: value }
    }
}

pub type UploadVideoChunkResponseJson = AppJson<String>;
// end RESPONSE DTOS

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
// end TABLE

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_video_request_validation() {
        // Valid request
        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
        };
        assert!(request.validate().is_ok());

        // Invalid title (too short)
        let request = CreateVideoRequest {
            title: "Short title".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
        };
        assert!(request.validate().is_err());

        // Invalid description (too short)
        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "Too short".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string(), "comedy".to_string()],
        };
        assert!(request.validate().is_err());

        // Invalid categories (too few)
        let request = CreateVideoRequest {
            title: "A valid video title that is long enough".to_string(),
            description: "This is a valid description that is intentionally made to be over one hundred characters long to pass the validation check definitely.".to_string(),
            visibility: Visibility::Public,
            categories: vec!["action".to_string()],
        };
        assert!(request.validate().is_err());
    }
}
