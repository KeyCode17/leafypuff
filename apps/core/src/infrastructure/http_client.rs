use std::time::Duration;

use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::domain::CoreError;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const DEVICE_HEADER: HeaderName = HeaderName::from_static("x-device-id");

pub fn plain(unreachable: &str) -> Result<Client, CoreError> {
    build(HeaderMap::new(), unreachable)
}

pub fn for_device(device_id: &str, unreachable: &str) -> Result<Client, CoreError> {
    let value =
        HeaderValue::from_str(device_id).map_err(|_| CoreError::Storage(unreachable.to_owned()))?;
    let mut headers = HeaderMap::new();
    headers.insert(DEVICE_HEADER, value);
    build(headers, unreachable)
}

fn build(headers: HeaderMap, unreachable: &str) -> Result<Client, CoreError> {
    Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .default_headers(headers)
        .build()
        .map_err(|_| CoreError::Storage(unreachable.to_owned()))
}
