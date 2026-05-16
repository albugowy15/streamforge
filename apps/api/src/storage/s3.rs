use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    config::{Credentials, Region},
};

use crate::config::Config;

pub struct S3(Client);

impl S3 {
    pub async fn new(config: &Config) -> Self {
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
        Self(Client::new(&shared_config))
    }

    pub fn get_client(&self) -> &Client {
        &self.0
    }
}
