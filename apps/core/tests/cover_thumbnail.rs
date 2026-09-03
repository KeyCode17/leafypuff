mod support;

use leafypuff_core::domain::crop::{COVER_MAX_HEIGHT, COVER_MAX_WIDTH, Framing};
use leafypuff_core::domain::{CoreError, ThumbnailMaker};
use leafypuff_core::infrastructure::ImageThumbnailer;

const SOURCE_WIDTH: u32 = 300;
const SOURCE_HEIGHT: u32 = 400;
const TOP_ROWS: u32 = 100;

#[test]
fn the_cover_is_a_three_by_two_crop_of_the_source() {
    let source = support::banded_jpeg(SOURCE_WIDTH, SOURCE_HEIGHT, TOP_ROWS);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");

    assert_eq!(support::dimensions(&cover), (300, 200));
    let (width, height) = support::dimensions(&cover);
    assert_eq!(u64::from(width) * 2, u64::from(height) * 3);
}

#[test]
fn the_cover_is_anchored_to_the_top_of_the_source() {
    let source = support::banded_jpeg(SOURCE_WIDTH, SOURCE_HEIGHT, TOP_ROWS);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");

    let picked = support::decode(&source);
    let drawn = support::decode(&cover);

    let inside_the_band = *drawn.get_pixel(150, 40);
    assert!(
        support::close(inside_the_band, *picked.get_pixel(150, 40)),
        "row 40 of the cover should still be row 40 of the source",
    );
    assert!(support::close(inside_the_band, support::TOP_BAND));

    let below_the_band = *drawn.get_pixel(150, 190);
    assert!(
        support::close(below_the_band, *picked.get_pixel(150, 190)),
        "row 190 of the cover should still be row 190 of the source",
    );
    assert!(support::close(below_the_band, support::BOTTOM_BAND));
}

#[test]
fn a_centred_crop_would_have_lost_the_top_band() {
    let source = support::banded_jpeg(SOURCE_WIDTH, SOURCE_HEIGHT, TOP_ROWS);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");
    let drawn = support::decode(&cover);

    let centred_row = (SOURCE_HEIGHT - 200) / 2;
    let picked = support::decode(&source);
    assert!(
        support::close(*picked.get_pixel(150, centred_row), support::BOTTOM_BAND),
        "the fixture must be banded so a centred crop reads differently",
    );
    assert!(support::close(*drawn.get_pixel(150, 0), support::TOP_BAND));
}

#[test]
fn a_large_photo_comes_down_to_the_cover_size() {
    let source = support::banded_jpeg(2400, 1600, 400);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");

    assert_eq!(
        support::dimensions(&cover),
        (COVER_MAX_WIDTH, COVER_MAX_HEIGHT)
    );

    let drawn = support::decode(&cover);
    assert!(support::close(*drawn.get_pixel(540, 60), support::TOP_BAND));
    assert!(support::close(
        *drawn.get_pixel(540, 660),
        support::BOTTOM_BAND
    ));
}

#[test]
fn a_wide_photo_keeps_its_full_height() {
    let source = support::banded_jpeg(900, 300, 150);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");

    assert_eq!(support::dimensions(&cover), (450, 300));
}

#[test]
fn a_payload_that_is_not_an_image_is_a_typed_photo_failure() {
    let refused = ImageThumbnailer.cover(&support::not_an_image());
    assert!(matches!(refused, Err(CoreError::Photo(_))));
}

#[test]
fn a_truncated_photo_is_a_typed_photo_failure() {
    let refused = ImageThumbnailer.cover(&support::truncated_jpeg());
    assert!(matches!(refused, Err(CoreError::Photo(_))));
}

const ROTATE_RIGHT_TO_STAND: u16 = 6;
const ROTATE_LEFT_TO_STAND: u16 = 8;

#[test]
fn a_photo_tagged_to_be_rotated_is_stood_up_before_the_cover_is_cut() {
    let source = support::banded_jpeg_oriented(400, 300, 150, ROTATE_RIGHT_TO_STAND);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");

    assert_eq!(support::dimensions(&cover), (300, 200));
    let drawn = support::decode(&cover);
    assert!(support::close(
        *drawn.get_pixel(250, 100),
        support::TOP_BAND
    ));
    assert!(support::close(
        *drawn.get_pixel(50, 100),
        support::BOTTOM_BAND
    ));
}

#[test]
fn the_tag_decides_which_way_the_photo_turns() {
    let source = support::banded_jpeg_oriented(400, 300, 150, ROTATE_LEFT_TO_STAND);
    let cover = ImageThumbnailer.cover(&source).expect("a cover is built");

    assert_eq!(support::dimensions(&cover), (300, 200));
    let drawn = support::decode(&cover);
    assert!(support::close(*drawn.get_pixel(50, 100), support::TOP_BAND));
    assert!(support::close(
        *drawn.get_pixel(250, 100),
        support::BOTTOM_BAND
    ));
}

#[test]
fn a_framed_cover_reads_the_framing_against_the_stood_up_photo() {
    let source = support::banded_jpeg_oriented(400, 300, 150, ROTATE_RIGHT_TO_STAND);
    let cover = ImageThumbnailer
        .framed_cover(&source, Framing::default())
        .expect("a framed cover is built");

    assert_eq!(support::dimensions(&cover), (300, 200));
    assert!(support::close(
        *support::decode(&cover).get_pixel(50, 50),
        support::BOTTOM_BAND
    ));
}

#[test]
fn a_framed_square_reads_the_framing_against_the_stood_up_photo() {
    let source = support::banded_jpeg_oriented(400, 300, 150, ROTATE_RIGHT_TO_STAND);
    let square = ImageThumbnailer
        .framed_square(&source, Framing::default())
        .expect("a framed square is built");

    assert_eq!(support::dimensions(&square), (300, 300));
    assert!(support::close(
        *support::decode(&square).get_pixel(50, 50),
        support::BOTTOM_BAND
    ));
}
