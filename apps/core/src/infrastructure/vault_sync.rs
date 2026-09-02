use data_encoding::BASE64;
use reqwest::Client;
use serde_json::{Value, json};

use super::http_client;
use super::http_error::{reached, refused};
use crate::domain::CoreError;
use crate::domain::crypto::{KeyVault, WrappedKey};

const KEYS_PATH: &str = "/v1/sync/keys";
const KIND_PASSPHRASE: &str = "passphrase";
const KIND_RECOVERY: &str = "recovery";
const NONCE_LEN: usize = 24;

const ERR_UNREACHABLE: &str = "The sync service could not be reached";
const ERR_SHAPE: &str = "The sync service answered an unexpected shape";
const ERR_SLOT: &str = "A stored key slot is the wrong length";

pub struct VaultSync {
    client: Client,
    base_url: String,
    access_token: String,
}

impl VaultSync {
    pub fn new(base_url: String, access_token: String, device_id: &str) -> Result<Self, CoreError> {
        Ok(Self {
            client: http_client::for_device(device_id, ERR_UNREACHABLE)?,
            base_url,
            access_token,
        })
    }

    pub async fn push(&self, vault: &KeyVault, updated_at_ms: i64) -> Result<(), CoreError> {
        let salt = BASE64.encode(&vault.passphrase_salt);
        for (kind, slot) in [
            (KIND_PASSPHRASE, &vault.passphrase_slot),
            (KIND_RECOVERY, &vault.recovery_slot),
        ] {
            self.put(kind, slot, &salt, updated_at_ms).await?;
        }
        Ok(())
    }

    pub async fn pull(&self) -> Result<Option<KeyVault>, CoreError> {
        let body = self.get().await?;
        let rows = body["data"].as_array().ok_or(shape())?;
        let passphrase = row(rows, KIND_PASSPHRASE);
        let recovery = row(rows, KIND_RECOVERY);
        let (Some(passphrase), Some(recovery)) = (passphrase, recovery) else {
            return Ok(None);
        };
        Ok(Some(KeyVault {
            passphrase_salt: decode(passphrase["salt"].as_str().ok_or(shape())?)?
                .as_slice()
                .try_into()
                .map_err(|_| CoreError::Storage(ERR_SLOT.to_owned()))?,
            passphrase_slot: slot(passphrase)?,
            recovery_slot: slot(recovery)?,
        }))
    }

    async fn put(
        &self,
        kind: &str,
        wrapped: &WrappedKey,
        salt: &str,
        updated_at_ms: i64,
    ) -> Result<(), CoreError> {
        let mut blob = wrapped.nonce.to_vec();
        blob.extend_from_slice(&wrapped.ciphertext);
        let response = self
            .client
            .put(format!("{}{KEYS_PATH}", self.base_url))
            .bearer_auth(&self.access_token)
            .json(&json!({
                "kind": kind,
                "blob": BASE64.encode(&blob),
                "salt": salt,
                "updated_at_ms": updated_at_ms,
            }))
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        refuse_unless_ok(response).await
    }

    async fn get(&self) -> Result<Value, CoreError> {
        let response = self
            .client
            .get(format!("{}{KEYS_PATH}", self.base_url))
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        let status = response.status();
        if !status.is_success() {
            return Err(refused(status, ERR_UNREACHABLE));
        }
        response
            .json()
            .await
            .map_err(|_| CoreError::Unreadable(ERR_SHAPE.to_owned()))
    }
}

async fn refuse_unless_ok(response: reqwest::Response) -> Result<(), CoreError> {
    if response.status().is_success() {
        return Ok(());
    }
    Err(refused(response.status(), ERR_UNREACHABLE))
}

fn row<'a>(rows: &'a [Value], kind: &str) -> Option<&'a Value> {
    rows.iter().find(|row| row["kind"].as_str() == Some(kind))
}

fn slot(row: &Value) -> Result<WrappedKey, CoreError> {
    let blob = decode(row["blob"].as_str().ok_or(shape())?)?;
    if blob.len() <= NONCE_LEN {
        return Err(CoreError::Storage(ERR_SLOT.to_owned()));
    }
    let (nonce, ciphertext) = blob.split_at(NONCE_LEN);
    Ok(WrappedKey {
        nonce: nonce
            .try_into()
            .map_err(|_| CoreError::Storage(ERR_SLOT.to_owned()))?,
        ciphertext: ciphertext.to_vec(),
    })
}

fn decode(raw: &str) -> Result<Vec<u8>, CoreError> {
    BASE64
        .decode(raw.as_bytes())
        .map_err(|_| CoreError::Unreadable(ERR_SHAPE.to_owned()))
}

fn shape() -> CoreError {
    CoreError::Unreadable(ERR_SHAPE.to_owned())
}
