use data_encoding::BASE64;
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};

use super::http_client;
use super::http_error::{reached, refused};
use crate::domain::{CoreError, PhotoKind};

const PROFILE_PATH: &str = "/v1/profile";
const VARIANT_ORIGINAL: &str = "original";
const VARIANT_DERIVATIVE: &str = "derivative";

const ERR_UNREACHABLE: &str = "The profile service could not be reached";
const ERR_SHAPE: &str = "The profile service answered an unexpected shape";

pub struct RemoteProfile {
    pub sealed_profile: Option<Vec<u8>>,
    pub avatar_photo_id: Option<String>,
    pub updated_at_ms: i64,
}

impl RemoteProfile {
    pub const fn empty() -> Self {
        Self {
            sealed_profile: None,
            avatar_photo_id: None,
            updated_at_ms: 0,
        }
    }

    fn borrowed(&self) -> Self {
        Self {
            sealed_profile: self.sealed_profile.clone(),
            avatar_photo_id: self.avatar_photo_id.clone(),
            updated_at_ms: self.updated_at_ms,
        }
    }
}

pub struct ProfileSync {
    client: Client,
    base_url: String,
    access_token: String,
}

impl ProfileSync {
    pub fn new(base_url: String, access_token: String, device_id: &str) -> Result<Self, CoreError> {
        Ok(Self {
            client: http_client::for_device(device_id, ERR_UNREACHABLE)?,
            base_url,
            access_token,
        })
    }

    pub async fn pull(&self) -> Result<RemoteProfile, CoreError> {
        let response = self
            .client
            .get(format!("{}{PROFILE_PATH}", self.base_url))
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(RemoteProfile::empty());
        }
        if !response.status().is_success() {
            return Err(unreachable(response.status()));
        }
        let body: Value = response
            .json()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        held(&body["data"])
    }

    pub async fn push(&self, wanted: &RemoteProfile) -> Result<RemoteProfile, CoreError> {
        let response = self
            .client
            .put(format!("{}{PROFILE_PATH}", self.base_url))
            .bearer_auth(&self.access_token)
            .json(&json!({
                "sealed_profile": wanted
                    .sealed_profile
                    .as_ref()
                    .map(|sealed| BASE64.encode(sealed)),
                "avatar_photo_id": wanted.avatar_photo_id,
                "updated_at_ms": wanted.updated_at_ms,
            }))
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(wanted.borrowed());
        }
        if !response.status().is_success() {
            return Err(unreachable(response.status()));
        }
        let body: Value = response
            .json()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        held(&body["data"])
    }

    pub async fn upload_avatar(&self, kind: PhotoKind, sealed: Vec<u8>) -> Result<(), CoreError> {
        let response = self
            .client
            .put(self.avatar_url(kind))
            .bearer_auth(&self.access_token)
            .body(sealed)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        if response.status().is_success() || response.status() == StatusCode::NOT_FOUND {
            return Ok(());
        }
        Err(unreachable(response.status()))
    }

    pub async fn download_avatar(&self, kind: PhotoKind) -> Result<Option<Vec<u8>>, CoreError> {
        let response = self
            .client
            .get(self.avatar_url(kind))
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

    fn avatar_url(&self, kind: PhotoKind) -> String {
        format!("{}{PROFILE_PATH}/avatar/{}", self.base_url, variant(kind))
    }
}

const fn variant(kind: PhotoKind) -> &'static str {
    match kind {
        PhotoKind::Original => VARIANT_ORIGINAL,
        PhotoKind::Cover => VARIANT_DERIVATIVE,
    }
}

fn held(data: &Value) -> Result<RemoteProfile, CoreError> {
    let sealed = match data["sealed_profile"].as_str() {
        Some(encoded) => Some(
            BASE64
                .decode(encoded.as_bytes())
                .map_err(|_| CoreError::Storage(ERR_SHAPE.to_owned()))?,
        ),
        None => None,
    };
    Ok(RemoteProfile {
        sealed_profile: sealed,
        avatar_photo_id: data["avatar_photo_id"].as_str().map(str::to_owned),
        updated_at_ms: data["updated_at_ms"].as_i64().unwrap_or_default(),
    })
}

fn unreachable(status: StatusCode) -> CoreError {
    refused(status, ERR_UNREACHABLE)
}
