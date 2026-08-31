mod support;

use chrono::{NaiveDate, TimeZone, Utc};
use leafypuff_core::application::ImportPhoto;
use leafypuff_core::domain::{COVER_ORDINAL, CoreError, PhotoKind, PhotoStore};
use leafypuff_core::infrastructure::{
    FilePhotoStore, ImageThumbnailer, KamadakExifReader, PlaintextSealer,
};

type Store = FilePhotoStore<PlaintextSealer>;

fn importer(dir: &tempfile::TempDir) -> ImportPhoto<Store, KamadakExifReader, ImageThumbnailer> {
    ImportPhoto::new(
        FilePhotoStore::new(dir.path().join("photos"), PlaintextSealer),
        KamadakExifReader,
        ImageThumbnailer,
    )
}

fn store(dir: &tempfile::TempDir) -> Store {
    FilePhotoStore::new(dir.path().join("photos"), PlaintextSealer)
}

#[test]
fn importing_writes_the_original_and_its_cover() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::banded_jpeg(300, 400, 100);

    let imported = importer(&dir).execute(&bytes).expect("the photo imports");

    assert_eq!(imported.ordinal, COVER_ORDINAL);
    assert!(std::path::Path::new(&imported.path).is_file());

    let held = store(&dir);
    assert_eq!(
        held.read(&imported.id, PhotoKind::Original)
            .expect("the original is stored"),
        bytes
    );
    let cover = held
        .read(&imported.id, PhotoKind::Cover)
        .expect("the cover is stored");
    assert_eq!(support::dimensions(&cover), (300, 200));
}

#[test]
fn the_stored_cover_still_shows_the_top_of_the_photo() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::banded_jpeg(300, 400, 100);

    let imported = importer(&dir).execute(&bytes).expect("the photo imports");
    let cover = store(&dir)
        .read(&imported.id, PhotoKind::Cover)
        .expect("the cover is stored");

    let picked = support::decode(&bytes);
    let drawn = support::decode(&cover);
    assert!(support::close(
        *drawn.get_pixel(150, 40),
        *picked.get_pixel(150, 40)
    ));
    assert!(support::close(*drawn.get_pixel(150, 40), support::TOP_BAND));
}

#[test]
fn a_tagged_photo_carries_its_capture_day_out_of_the_import() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::jpeg_taken_on(b"2026:07:04 09:12:33");

    let imported = importer(&dir).execute(&bytes).expect("the photo imports");

    let expected = NaiveDate::from_ymd_opt(2026, 7, 4)
        .and_then(|day| day.and_hms_opt(0, 0, 0))
        .map(|at| Utc.from_utc_datetime(&at));
    assert_eq!(imported.taken_at, expected);
}

#[test]
fn an_untagged_photo_imports_without_a_capture_day() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::solid_jpeg(300, 400);

    let imported = importer(&dir).execute(&bytes).expect("the photo imports");

    assert_eq!(imported.taken_at, None);
}

#[test]
fn every_import_gets_its_own_identity() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let bytes = support::solid_jpeg(300, 400);
    let import = importer(&dir);

    let first = import.execute(&bytes).expect("the photo imports");
    let second = import.execute(&bytes).expect("the photo imports again");

    assert_ne!(first.id, second.id);
    assert_ne!(first.path, second.path);
}

#[test]
fn a_payload_that_is_not_an_image_writes_nothing() {
    let dir = tempfile::tempdir().expect("a temp dir");

    let refused = importer(&dir).execute(&support::not_an_image());

    assert!(matches!(refused, Err(CoreError::Photo(_))));
    assert!(!dir.path().join("photos").exists());
}
