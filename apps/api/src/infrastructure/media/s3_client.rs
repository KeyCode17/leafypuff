use aws_credential_types::Credentials;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, Config as S3Config};
use aws_smithy_async::rt::sleep::default_async_sleep;

use crate::infrastructure::Config;

const REGION: &str = "garage";
const PROVIDER: &str = "leafypuff-config";

pub fn build_s3_client(config: &Config) -> Client {
    let credentials = Credentials::new(
        config.s3_access_key.clone(),
        config.s3_secret_key.clone(),
        None,
        None,
        PROVIDER,
    );
    let mut builder = S3Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .endpoint_url(endpoint(&config.s3_endpoint))
        .credentials_provider(credentials)
        // Garage speaks path style only. Virtual-hosted style would resolve a bucket
        // subdomain that has no DNS record on loopback.
        .force_path_style(true);
    builder.set_sleep_impl(default_async_sleep());
    Client::from_conf(builder.build())
}

fn endpoint(configured: &str) -> String {
    if configured.starts_with("http://") || configured.starts_with("https://") {
        return configured.to_owned();
    }
    format!("http://{configured}")
}
