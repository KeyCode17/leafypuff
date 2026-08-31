mod support;

use leafypuff_core::domain::crypto::ContentKey;
use leafypuff_core::domain::{CoreError, PhotoKind, PhotoStore};
use leafypuff_core::infrastructure::{FilePhotoStore, XChaChaSealer};

const PHOTO_ID: &str = "3f2a91c0-0000-4000-8000-0000000000aa";

fn store_in(dir: &tempfile::TempDir) -> FilePhotoStore<XChaChaSealer> {
    let key = ContentKey::generate().expect("entropy");
    FilePhotoStore::new(dir.path().join("photos"), XChaChaSealer::new(key))
}

#[test]
fn a_photo_lands_on_disk_as_ciphertext_and_reads_back_unchanged() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);
    let bytes = support::solid_jpeg(64, 48);

    let path = store
        .write(PHOTO_ID, PhotoKind::Original, &bytes)
        .expect("the original is written");

    let on_disk = std::fs::read(&path).expect("the file exists");
    assert_ne!(on_disk, bytes);
    assert!(!on_disk.starts_with(&bytes[..16]));
    assert!(
        on_disk
            .windows(bytes.len())
            .all(|window| window != bytes.as_slice())
    );
    assert_eq!(
        store
            .read(PHOTO_ID, PhotoKind::Original)
            .expect("reads back"),
        bytes
    );
}

#[test]
fn the_cover_and_the_original_are_sealed_under_different_labels() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);

    let original = store
        .write(PHOTO_ID, PhotoKind::Original, b"the picked bytes")
        .expect("the original is written");
    let cover = store
        .write(PHOTO_ID, PhotoKind::Cover, b"the cover bytes")
        .expect("the cover is written");
    std::fs::copy(&original, &cover).expect("the cover file is overwritten");

    let moved = store.read(PHOTO_ID, PhotoKind::Cover);
    assert!(matches!(moved, Err(CoreError::Crypto(_))));
}

#[test]
fn a_truncated_file_is_a_typed_failure_rather_than_a_panic() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let store = store_in(&dir);

    let path = store
        .write(PHOTO_ID, PhotoKind::Original, b"the picked bytes")
        .expect("the original is written");
    std::fs::write(&path, b"short").expect("the file is truncated");

    let truncated = store.read(PHOTO_ID, PhotoKind::Original);
    assert!(matches!(truncated, Err(CoreError::Crypto(_))));
}

#[test]
fn another_content_key_cannot_read_the_photos_of_this_one() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::solid_jpeg(64, 48);
    store_in(&dir)
        .write(PHOTO_ID, PhotoKind::Original, &bytes)
        .expect("the original is written");

    let stranger = store_in(&dir).read(PHOTO_ID, PhotoKind::Original);
    assert!(matches!(stranger, Err(CoreError::Crypto(_))));
}

#[test]
fn the_error_a_failed_open_returns_carries_no_plaintext_and_no_lengths() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::solid_jpeg(64, 48);
    store_in(&dir)
        .write(PHOTO_ID, PhotoKind::Original, &bytes)
        .expect("the original is written");

    let Err(refused) = store_in(&dir).read(PHOTO_ID, PhotoKind::Original) else {
        panic!("a stranger key must not open the photo");
    };
    assert_eq!(refused.to_string(), "Crypto failure: Decryption failed");
}
