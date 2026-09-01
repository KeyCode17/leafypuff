use std::sync::{Arc, RwLock};

use crate::domain::crypto::{
    ContentKey, FieldContext, KEY_LEN, SealedField, WrappedKey, open, open_blob, seal, seal_blob,
    seal_for_device,
};
use crate::domain::error::ERR_VAULT_LOCKED;
use crate::domain::{ContentSealer, CoreError, EntryId, FieldSealer};

/// The one handle every seal and open goes through. Holds the content key once a vault is
/// unlocked and nothing before that, so a write attempted while locked fails rather than
/// silently storing plaintext.
#[derive(Clone, Default)]
pub struct VaultSealer {
    key: Arc<RwLock<Option<ContentKey>>>,
}

impl VaultSealer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn unlock(&self, key: ContentKey) -> Result<(), CoreError> {
        let mut held = self.key.write().map_err(|_| CoreError::Locked)?;
        *held = Some(key);
        Ok(())
    }

    pub fn lock(&self) -> Result<(), CoreError> {
        let mut held = self.key.write().map_err(|_| CoreError::Locked)?;
        *held = None;
        Ok(())
    }

    pub fn is_unlocked(&self) -> bool {
        self.key.read().is_ok_and(|held| held.is_some())
    }

    /// Wraps the held content key under a key the device keeps in hardware. The content key
    /// itself never leaves this type -- the caller gets ciphertext it cannot open without the
    /// same device key.
    pub fn seal_for_device(&self, device_key: &[u8; KEY_LEN]) -> Result<WrappedKey, CoreError> {
        self.with_key(|key| Ok(seal_for_device(device_key, key)?))
    }

    fn with_key<T>(
        &self,
        task: impl FnOnce(&ContentKey) -> Result<T, CoreError>,
    ) -> Result<T, CoreError> {
        let held = self.key.read().map_err(|_| CoreError::Locked)?;
        let key = held.as_ref().ok_or(CoreError::Locked)?;
        task(key)
    }
}

impl FieldSealer for VaultSealer {
    fn seal_field(
        &self,
        entry_id: EntryId,
        field_name: &str,
        field_updated_at_ms: i64,
        plain: &str,
    ) -> Result<SealedField, CoreError> {
        self.with_key(|key| {
            let context = FieldContext {
                entry_id,
                field_name,
                field_updated_at_ms,
            };
            Ok(seal(key, &context, plain.as_bytes())?)
        })
    }

    fn open_field(
        &self,
        entry_id: EntryId,
        field_name: &str,
        field_updated_at_ms: i64,
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Result<String, CoreError> {
        let nonce: [u8; 24] = nonce
            .try_into()
            .map_err(|_| CoreError::Storage(ERR_VAULT_LOCKED.to_owned()))?;
        self.with_key(|key| {
            let context = FieldContext {
                entry_id,
                field_name,
                field_updated_at_ms,
            };
            let sealed = SealedField {
                nonce,
                ciphertext: ciphertext.to_vec(),
            };
            let plain = open(key, &context, &sealed)?;
            String::from_utf8(plain).map_err(|_| CoreError::Storage(ERR_VAULT_LOCKED.to_owned()))
        })
    }
}

impl ContentSealer for VaultSealer {
    fn seal(&self, label: &str, plain: &[u8]) -> Result<Vec<u8>, CoreError> {
        self.with_key(|key| Ok(seal_blob(key, label, plain)?))
    }

    fn open(&self, label: &str, sealed: &[u8]) -> Result<Vec<u8>, CoreError> {
        self.with_key(|key| Ok(open_blob(key, label, sealed)?))
    }
}
