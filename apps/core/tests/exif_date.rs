mod support;

use chrono::NaiveDate;
use leafypuff_core::domain::ExifReader;
use leafypuff_core::infrastructure::KamadakExifReader;

#[test]
fn a_tagged_photo_reports_the_day_it_was_taken() {
    let read = KamadakExifReader
        .taken_on(&support::jpeg_taken_on(b"2026:07:04 09:12:33"))
        .expect("a tagged photo is readable");

    assert_eq!(read, NaiveDate::from_ymd_opt(2026, 7, 4));
}

#[test]
fn a_photo_without_exif_reports_nothing() {
    let read = KamadakExifReader
        .taken_on(&support::solid_jpeg(64, 48))
        .expect("an untagged photo is not a failure");

    assert_eq!(read, None);
}

#[test]
fn a_corrupt_exif_block_reports_nothing() {
    let read = KamadakExifReader
        .taken_on(&support::jpeg_with_broken_exif(b"2026:07:04 09:12:33"))
        .expect("a corrupt block is not a failure");

    assert_eq!(read, None);
}

#[test]
fn a_truncated_photo_reports_nothing() {
    let read = KamadakExifReader
        .taken_on(&support::truncated_jpeg())
        .expect("a truncated file is not a failure");

    assert_eq!(read, None);
}

#[test]
fn a_file_that_is_not_an_image_reports_nothing() {
    let read = KamadakExifReader
        .taken_on(&support::not_an_image())
        .expect("a non-image is not a failure");

    assert_eq!(read, None);
}

#[test]
fn an_empty_payload_reports_nothing() {
    let read = KamadakExifReader
        .taken_on(&[])
        .expect("an empty payload is not a failure");

    assert_eq!(read, None);
}

#[test]
fn an_unparseable_stamp_reports_nothing() {
    let read = KamadakExifReader
        .taken_on(&support::jpeg_taken_on(b"not a timestamp yet"))
        .expect("a malformed stamp is not a failure");

    assert_eq!(read, None);
}
