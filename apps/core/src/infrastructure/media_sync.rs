use std::time::Duration;

use reqwest::{Client, StatusCode};

use super::http_error::reached;
use crate::domain::{CoreError, PhotoKind};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

const MEDIA_PATH: &str = "/v1/media";
const VARIANT_ORIGINAL: &str = "original";
const VARIANT_DERIVATIVE: &str = "derivative";

const ERR_UNREACHABLE: &str = "The media service could not be reached";

pub struct MediaSync {
    client: Client,
    base_url: String,
    access_token: String,
}

impl MediaSync {
    pub fn new(base_url: String, access_token: String) -> Result<Self, CoreError> {
        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|_| CoreError::Storage(ERR_UNREACHABLE.to_owned()))?;
        Ok(Self {
            client,
            base_url,
            access_token,
        })
    }

    pub async fn upload(
        &self,
        photo_id: &str,
        kind: PhotoKind,
        sealed: Vec<u8>,
    ) -> Result<(), CoreError> {
        let response = self
            .client
            .put(self.url(photo_id, kind))
            .bearer_auth(&self.access_token)
            .body(sealed)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        if response.status().is_success() {
            return Ok(());
        }
        Err(unreachable(response.status()))
    }

    pub async fn download(
        &self,
        photo_id: &str,
        kind: PhotoKind,
    ) -> Result<Option<Vec<u8>>, CoreError> {
        let response = self
            .client
            .get(self.url(photo_id, kind))
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !response.status().is_success() {
            return Err(unreachable(response.status()));
        }
        let bytes = response
            .bytes()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        Ok(Some(bytes.to_vec()))
    }

    fn url(&self, photo_id: &str, kind: PhotoKind) -> String {
        format!("{}{MEDIA_PATH}/{photo_id}/{}", self.base_url, variant(kind))
    }
}

const fn variant(kind: PhotoKind) -> &'static str {
    match kind {
        PhotoKind::Original => VARIANT_ORIGINAL,
        PhotoKind::Cover => VARIANT_DERIVATIVE,
    }
}

fn unreachable(status: StatusCode) -> CoreError {
    CoreError::Storage(format!("{ERR_UNREACHABLE}: {}", status.as_u16()))
}
