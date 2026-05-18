use serde::{Deserialize, Serialize};

use crate::json::{AppJson, JsonData};

#[derive(Debug, Clone)]
pub struct Video {
    pub id: Option<i64>,
    pub name: String,
}

impl Video {
    pub fn new(id: Option<i64>, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: String,
    pub visibility: Visibility,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub id: i64,
    pub name: String,
}

pub type CreateVideoResponse = AppJson<JsonData<VideoResponse>>;

impl From<VideoResponse> for JsonData<VideoResponse> {
    fn from(value: VideoResponse) -> Self {
        JsonData { data: value }
    }
}

impl From<Video> for VideoResponse {
    fn from(item: Video) -> Self {
        Self {
            id: item.id.unwrap_or(0),
            name: item.name,
        }
    }
}
