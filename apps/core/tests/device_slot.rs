#![cfg(feature = "test-support")]

use leafypuff_core::domain::crypto::{
    ContentKey, KEY_LEN, KeyVault, RecoveryCode, open_for_device, seal_for_device,
};

const DEVICE_KEY: [u8; KEY_LEN] = [0x11; KEY_LEN];
const OTHER_KEY: [u8; KEY_LEN] = [0x22; KEY_LEN];
const PASSPHRASE: &str = "a passphrase long enough to matter";

fn vault() -> (KeyVault, ContentKey) {
    let code = RecoveryCode::generate().expect("entropy must be available");
    KeyVault::create(PASSPHRASE, &code).expect("a vault is created")
}

#[test]
fn a_device_key_opens_what_it_sealed() {
    let (_, content) = vault();
    let wrapped = seal_for_device(&DEVICE_KEY, &content).expect("the key wraps");

    let reopened = open_for_device(&DEVICE_KEY, &wrapped).expect("the key unwraps");

    assert_eq!(format!("{reopened:?}"), "ContentKey(redacted)");
}

#[test]
fn another_device_key_opens_nothing() {
    let (_, content) = vault();
    let wrapped = seal_for_device(&DEVICE_KEY, &content).expect("the key wraps");

    assert!(open_for_device(&OTHER_KEY, &wrapped).is_err());
}

#[test]
fn the_device_slot_carries_the_same_content_key_as_the_passphrase_slot() {
    let code = RecoveryCode::generate().expect("entropy must be available");
    let (held, content) = KeyVault::create(PASSPHRASE, &code).expect("a vault is created");
    let wrapped = seal_for_device(&DEVICE_KEY, &content).expect("the key wraps");

    let from_device = open_for_device(&DEVICE_KEY, &wrapped).expect("the device slot opens");
    let from_passphrase = held
        .unlock_with_passphrase(PASSPHRASE)
        .expect("the passphrase slot opens");

    let context = leafypuff_core::domain::crypto::FieldContext {
        entry_id: leafypuff_core::domain::EntryId::new(),
        field_name: "title",
        field_updated_at_ms: 0,
    };
    let sealed = leafypuff_core::domain::crypto::seal(&from_device, &context, b"a quiet morning")
        .expect("sealing under the device copy");
    let plain = leafypuff_core::domain::crypto::open(&from_passphrase, &context, &sealed)
        .expect("opening under the passphrase copy");

    assert_eq!(plain, b"a quiet morning");
}

#[cfg(feature = "sqlite")]
mod stored {
    use leafypuff_core::domain::crypto::{RecoveryCode, WrappedKey, seal_for_device};
    use leafypuff_core::infrastructure::{SqliteDeviceSlotStore, db};

    use super::{DEVICE_KEY, PASSPHRASE};

    async fn store() -> (tempfile::TempDir, SqliteDeviceSlotStore) {
        let dir = tempfile::tempdir().expect("a temp dir");
        let path = dir
            .path()
            .join("diary.sqlite")
            .to_string_lossy()
            .into_owned();
        let connection = db::open(&path).await.expect("a temp file opens");
        db::run_migrations(&connection)
            .await
            .expect("migrations apply");
        (dir, SqliteDeviceSlotStore::new(connection))
    }

    fn wrapped() -> WrappedKey {
        let code = RecoveryCode::generate().expect("entropy must be available");
        let (_, content) = leafypuff_core::domain::crypto::KeyVault::create(PASSPHRASE, &code)
            .expect("a vault is created");
        seal_for_device(&DEVICE_KEY, &content).expect("the key wraps")
    }

    #[tokio::test]
    async fn a_fresh_device_holds_no_slot() {
        let (_dir, store) = store().await;

        assert!(store.read().await.expect("the read succeeds").is_none());
    }

    #[tokio::test]
    async fn a_written_slot_reads_back_byte_for_byte() {
        let (_dir, store) = store().await;
        let held = wrapped();

        store.replace(&held).await.expect("the slot is written");
        let read = store
            .read()
            .await
            .expect("the read succeeds")
            .expect("a slot is present");

        assert_eq!(read.nonce, held.nonce);
        assert_eq!(read.ciphertext, held.ciphertext);
    }

    #[tokio::test]
    async fn writing_twice_keeps_one_slot_and_forgetting_leaves_none() {
        let (_dir, store) = store().await;

        store.replace(&wrapped()).await.expect("the first write");
        store.replace(&wrapped()).await.expect("the second write");
        assert!(store.read().await.expect("the read succeeds").is_some());

        store.forget().await.expect("the slot is dropped");
        assert!(store.read().await.expect("the read succeeds").is_none());
    }
}
