#![cfg(feature = "ffi")]

mod support;

use std::sync::Arc;

use leafypuff_core::ffi::{LeafyPuffCore, LeafyPuffCoreError};

const VAULT_PASSPHRASE: &str = "a passphrase only this test uses";

async fn core() -> (tempfile::TempDir, Arc<LeafyPuffCore>) {
    let dir = tempfile::tempdir().expect("a temp dir");
    let path = dir
        .path()
        .join("diary.sqlite")
        .to_string_lossy()
        .into_owned();
    let core = LeafyPuffCore::new(path).await.expect("the core opens");
    core.create_vault(VAULT_PASSPHRASE.to_owned())
        .await
        .expect("the vault is created");
    (dir, core)
}

#[tokio::test]
async fn an_imported_photo_comes_back_with_its_capture_day() {
    let (_dir, core) = core().await;

    let imported = core
        .import_photo(support::jpeg_taken_on(b"2026:07:04 09:12:33"))
        .expect("the photo imports");

    assert_eq!(imported.ordinal, 0);
    assert_eq!(
        imported.taken_at.as_deref(),
        Some("2026-07-04T00:00:00+00:00")
    );
    assert!(!imported.path.is_empty());
}

#[tokio::test]
async fn the_cover_of_an_imported_photo_is_a_three_by_two_jpeg() {
    let (_dir, core) = core().await;

    let imported = core
        .import_photo(support::banded_jpeg(300, 400, 100))
        .expect("the photo imports");
    let cover = core
        .cover_thumbnail(imported.id)
        .expect("the cover is fetched");

    let (width, height) = support::dimensions(&cover);
    assert_eq!((width, height), (300, 200));

    let drawn = support::decode(&cover);
    assert!(support::close(*drawn.get_pixel(150, 40), support::TOP_BAND));
}

#[tokio::test]
async fn the_capture_day_reads_straight_off_the_picked_bytes() {
    let (_dir, core) = core().await;

    let tagged = core
        .photo_taken_on(support::jpeg_taken_on(b"2026:07:04 09:12:33"))
        .expect("a tagged photo is readable");
    assert_eq!(tagged.as_deref(), Some("2026-07-04"));

    let untagged = core
        .photo_taken_on(support::solid_jpeg(64, 48))
        .expect("an untagged photo is not a failure");
    assert_eq!(untagged, None);

    let nonsense = core
        .photo_taken_on(support::not_an_image())
        .expect("a non-image is not a failure");
    assert_eq!(nonsense, None);
}

#[tokio::test]
async fn a_payload_that_is_not_an_image_is_refused_at_the_boundary() {
    let (_dir, core) = core().await;

    let refused = core.import_photo(support::not_an_image());
    assert!(matches!(refused, Err(LeafyPuffCoreError::Photo { .. })));
}

#[tokio::test]
async fn a_cover_that_was_never_imported_is_refused_at_the_boundary() {
    let (_dir, core) = core().await;

    let missing = core.cover_thumbnail("11111111-1111-4111-8111-111111111111".to_owned());
    assert!(matches!(missing, Err(LeafyPuffCoreError::Photo { .. })));

    let unsafe_id = core.cover_thumbnail("../diary.sqlite".to_owned());
    assert!(matches!(unsafe_id, Err(LeafyPuffCoreError::Invalid { .. })));
}
