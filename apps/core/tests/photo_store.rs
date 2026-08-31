mod support;

use std::path::PathBuf;

use leafypuff_core::domain::{ContentSealer, CoreError, PhotoKind, PhotoStore};
use leafypuff_core::infrastructure::{FilePhotoStore, PlaintextSealer};

const PHOTO_ID: &str = "3f2a91c0-0000-4000-8000-0000000000aa";

struct FlippingSealer;

impl ContentSealer for FlippingSealer {
    fn seal(&self, label: &str, plain: &[u8]) -> Result<Vec<u8>, CoreError> {
        let mut out = label.as_bytes().to_vec();
        out.extend(plain.iter().map(|byte| byte ^ 0x5A));
        Ok(out)
    }

    fn open(&self, label: &str, sealed: &[u8]) -> Result<Vec<u8>, CoreError> {
        let Some(body) = sealed.strip_prefix(label.as_bytes()) else {
            return Err(CoreError::Crypto("sealed under another label".to_owned()));
        };
        Ok(body.iter().map(|byte| byte ^ 0x5A).collect())
    }
}

fn store_in(dir: &tempfile::TempDir) -> FilePhotoStore<PlaintextSealer> {
    FilePhotoStore::new(dir.path().join("photos"), PlaintextSealer)
}

#[test]
fn a_written_blob_reads_back_unchanged() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);
    let bytes = support::solid_jpeg(48, 32);

    let path = store
        .write(PHOTO_ID, PhotoKind::Original, &bytes)
        .expect("the original is written");
    store
        .write(PHOTO_ID, PhotoKind::Cover, b"cover bytes")
        .expect("the cover is written");

    assert!(PathBuf::from(&path).is_file());
    assert_eq!(
        store
            .read(PHOTO_ID, PhotoKind::Original)
            .expect("reads back"),
        bytes
    );
    assert_eq!(
        store.read(PHOTO_ID, PhotoKind::Cover).expect("reads back"),
        b"cover bytes"
    );
}

#[test]
fn the_two_kinds_never_share_a_file() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);

    let original = store
        .write(PHOTO_ID, PhotoKind::Original, b"one")
        .expect("the original is written");
    let cover = store
        .write(PHOTO_ID, PhotoKind::Cover, b"two")
        .expect("the cover is written");

    assert_ne!(original, cover);
}

#[test]
fn the_sealer_is_the_only_thing_between_the_bytes_and_the_file() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = FilePhotoStore::new(dir.path().join("photos"), FlippingSealer);
    let bytes = support::solid_jpeg(48, 32);

    let path = store
        .write(PHOTO_ID, PhotoKind::Original, &bytes)
        .expect("the original is written");

    let on_disk = std::fs::read(&path).expect("the file exists");
    assert_ne!(on_disk, bytes);
    assert!(on_disk.starts_with(b"photo:"));
    assert_eq!(
        store
            .read(PHOTO_ID, PhotoKind::Original)
            .expect("reads back"),
        bytes
    );
}

#[test]
fn an_id_that_could_escape_the_photo_directory_is_refused() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);

    for id in ["../secrets", "a/b", "", "photo id", "..\\escape"] {
        let refused = store.write(id, PhotoKind::Original, b"x");
        assert!(
            matches!(refused, Err(CoreError::Invalid(_))),
            "{id} should not be a storage name"
        );
    }
}

#[test]
fn a_photo_that_was_never_imported_is_a_typed_photo_failure() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);

    let missing = store.read(PHOTO_ID, PhotoKind::Cover);
    assert!(matches!(missing, Err(CoreError::Photo(_))));
}

#[test]
fn the_store_lands_beside_the_database_it_was_opened_with() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let database = dir
        .path()
        .join("diary.sqlite")
        .to_string_lossy()
        .into_owned();

    let store = FilePhotoStore::beside(&database, PlaintextSealer);
    assert_eq!(store.root(), dir.path().join("photos"));
}
