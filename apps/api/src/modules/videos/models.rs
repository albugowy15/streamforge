use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVideoRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub id: i64,
    pub name: String,
}

impl From<Video> for VideoResponse {
    fn from(item: Video) -> Self {
        Self {
            id: item.id.unwrap_or(0),
            name: item.name,
        }
    }
}
