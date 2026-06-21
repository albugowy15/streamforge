use std::env::{self, VarError};

pub struct Config {
    pub database_url: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint_url: String,
    pub bucket: String,
}

impl Config {
    pub fn from_env() -> Result<Self, VarError> {
        let region = env::var("S3_REGION")?;
        let access_key_id = env::var("S3_ACCESS_KEY_ID")?;
        let secret_access_key = env::var("S3_SECRET_ACCESS_KEY")?;
        let endpoint_url = env::var("S3_ENDPOINT_URL")?;
        let bucket = env::var("S3_BUCKET")?;
        let database_url = env::var("DATABASE_URL")?;
        Ok(Self {
            database_url,
            region,
            access_key_id,
            secret_access_key,
            endpoint_url,
            bucket,
        })
    }
}
