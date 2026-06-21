use anyhow::Context;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    config::{Credentials, Region},
};
use tracing::{debug, info};

use crate::config::Config;

pub struct S3 {
    client: Client,
    bucket: String,
}

impl S3 {
    pub async fn new(config: &Config) -> Self {
        debug!(
            region = %config.region,
            endpoint_url = %config.endpoint_url,
            bucket = %config.bucket,
            "configuring S3-compatible client"
        );

        let credentials = Credentials::new(
            &config.access_key_id,
            &config.secret_access_key,
            None,
            None,
            "rustfs",
        );
        let region = Region::new(config.region.clone());
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region)
            .credentials_provider(credentials)
            .endpoint_url(&config.endpoint_url)
            .load()
            .await;
        let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();
        Self {
            client: Client::from_conf(s3_config),
            bucket: config.bucket.clone(),
        }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub async fn ensure_bucket_exists(&self) -> anyhow::Result<()> {
        debug!(bucket = %self.bucket, "checking S3 bucket");

        if self
            .client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .is_ok()
        {
            debug!(bucket = %self.bucket, "S3 bucket already exists");
            return Ok(());
        }

        info!(bucket = %self.bucket, "S3 bucket missing; creating it");

        self.client
            .create_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .with_context(|| format!("failed to create S3 bucket {}", self.bucket))?;

        info!(bucket = %self.bucket, "S3 bucket created");

        Ok(())
    }
}
