use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use crate::domain::CoreError;
use crate::domain::crypto::{KeyVault, WrappedKey};
use crate::domain::error::{ERR_VAULT_ABSENT, ERR_VAULT_PRESENT};

use super::entity::{device_slot, vault};

const ONLY_ROW: i32 = 1;

pub struct SqliteVaultStore {
    connection: DatabaseConnection,
}

impl SqliteVaultStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn read(&self) -> Result<KeyVault, CoreError> {
        let row = vault::Entity::find_by_id(ONLY_ROW)
            .one(&self.connection)
            .await?
            .ok_or_else(|| CoreError::Invalid(ERR_VAULT_ABSENT.to_owned()))?;
        Ok(KeyVault {
            passphrase_salt: row
                .passphrase_salt
                .as_slice()
                .try_into()
                .map_err(|_| CoreError::Storage(ERR_VAULT_ABSENT.to_owned()))?,
            passphrase_slot: wrapped(row.passphrase_nonce, row.passphrase_ciphertext)?,
            recovery_slot: wrapped(row.recovery_nonce, row.recovery_ciphertext)?,
        })
    }

    pub async fn exists(&self) -> Result<bool, CoreError> {
        Ok(vault::Entity::find_by_id(ONLY_ROW)
            .one(&self.connection)
            .await?
            .is_some())
    }

    pub async fn create(&self, held: &KeyVault) -> Result<(), CoreError> {
        if self.exists().await? {
            return Err(CoreError::Invalid(ERR_VAULT_PRESENT.to_owned()));
        }
        self.write(held).await
    }

    pub async fn replace(&self, held: &KeyVault) -> Result<(), CoreError> {
        vault::Entity::delete_by_id(ONLY_ROW)
            .exec(&self.connection)
            .await?;
        self.write(held).await
    }

    async fn write(&self, held: &KeyVault) -> Result<(), CoreError> {
        vault::Entity::insert(vault::ActiveModel {
            id: ActiveValue::Set(ONLY_ROW),
            passphrase_salt: ActiveValue::Set(held.passphrase_salt.to_vec()),
            passphrase_nonce: ActiveValue::Set(held.passphrase_slot.nonce.to_vec()),
            passphrase_ciphertext: ActiveValue::Set(held.passphrase_slot.ciphertext.clone()),
            recovery_nonce: ActiveValue::Set(held.recovery_slot.nonce.to_vec()),
            recovery_ciphertext: ActiveValue::Set(held.recovery_slot.ciphertext.clone()),
            created_at: ActiveValue::Set(Utc::now().to_rfc3339()),
        })
        .exec(&self.connection)
        .await?;
        Ok(())
    }
}

fn wrapped(nonce: Vec<u8>, ciphertext: Vec<u8>) -> Result<WrappedKey, CoreError> {
    Ok(WrappedKey {
        nonce: nonce
            .as_slice()
            .try_into()
            .map_err(|_| CoreError::Storage(ERR_VAULT_ABSENT.to_owned()))?,
        ciphertext,
    })
}

/// The device-bound copy of the content key. Separate from SqliteVaultStore on purpose: the vault
/// is what travels to the server, and this row is what must never leave the device.
pub struct SqliteDeviceSlotStore {
    connection: DatabaseConnection,
}

impl SqliteDeviceSlotStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn read(&self) -> Result<Option<WrappedKey>, CoreError> {
        let Some(row) = device_slot::Entity::find_by_id(ONLY_ROW)
            .one(&self.connection)
            .await?
        else {
            return Ok(None);
        };
        Ok(Some(wrapped(row.nonce, row.ciphertext)?))
    }

    pub async fn replace(&self, held: &WrappedKey) -> Result<(), CoreError> {
        device_slot::Entity::delete_by_id(ONLY_ROW)
            .exec(&self.connection)
            .await?;
        device_slot::Entity::insert(device_slot::ActiveModel {
            id: ActiveValue::Set(ONLY_ROW),
            nonce: ActiveValue::Set(held.nonce.to_vec()),
            ciphertext: ActiveValue::Set(held.ciphertext.clone()),
            created_at: ActiveValue::Set(Utc::now().to_rfc3339()),
        })
        .exec(&self.connection)
        .await?;
        Ok(())
    }

    pub async fn forget(&self) -> Result<(), CoreError> {
        device_slot::Entity::delete_by_id(ONLY_ROW)
            .exec(&self.connection)
            .await?;
        Ok(())
    }
}
