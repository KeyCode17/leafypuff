use leafypuff_core::domain::CoreError;
use leafypuff_core::domain::crop::{
    COVER_MAX_HEIGHT, COVER_MAX_WIDTH, CropBox, cover_size, top_anchored_cover,
};

const SIZES: [(u32, u32); 8] = [
    (3, 2),
    (4, 3),
    (300, 400),
    (301, 401),
    (999, 667),
    (1000, 400),
    (4000, 3000),
    (u32::MAX, u32::MAX),
];

#[test]
fn every_crop_is_exactly_three_by_two() {
    for (width, height) in SIZES {
        let frame = top_anchored_cover(width, height).expect("a crop fits");
        assert_eq!(
            u64::from(frame.width) * 2,
            u64::from(frame.height) * 3,
            "{width}x{height} produced {}x{}",
            frame.width,
            frame.height,
        );
        assert!(frame.width <= width && frame.height <= height);
        assert!(frame.x + frame.width <= width);
        assert!(frame.y + frame.height <= height);
    }
}

#[test]
fn a_tall_photo_is_anchored_to_the_top_not_the_centre() {
    let frame = top_anchored_cover(300, 400).expect("a crop fits");

    assert_eq!(
        frame,
        CropBox {
            x: 0,
            y: 0,
            width: 300,
            height: 200,
        }
    );
    assert_ne!(frame.y, (400 - frame.height) / 2);
}

#[test]
fn a_wide_photo_keeps_its_full_height_and_centres_across() {
    let frame = top_anchored_cover(900, 300).expect("a crop fits");

    assert_eq!(
        frame,
        CropBox {
            x: 225,
            y: 0,
            width: 450,
            height: 300,
        }
    );
}

#[test]
fn an_odd_photo_never_leans_past_a_single_pixel() {
    let frame = top_anchored_cover(301, 401).expect("a crop fits");

    assert_eq!(frame.width, 300);
    assert_eq!(frame.height, 200);
    assert_eq!(frame.y, 0);
}

#[test]
fn a_photo_smaller_than_one_cover_unit_is_rejected() {
    for (width, height) in [(2u32, 2u32), (300, 1), (0, 0), (1, 900)] {
        let refused = top_anchored_cover(width, height);
        assert!(
            matches!(refused, Err(CoreError::Photo(_))),
            "{width}x{height} should not crop"
        );
    }
}

#[test]
fn a_large_crop_comes_down_to_the_cover_size_and_a_small_one_stays() {
    let large = top_anchored_cover(4000, 3000).expect("a crop fits");
    assert_eq!(cover_size(large), (COVER_MAX_WIDTH, COVER_MAX_HEIGHT));

    let small = top_anchored_cover(300, 400).expect("a crop fits");
    assert_eq!(cover_size(small), (300, 200));
}
