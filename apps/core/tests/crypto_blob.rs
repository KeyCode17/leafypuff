use leafypuff_core::domain::crypto::{ContentKey, open_blob, seal_blob};

const ORIGINAL: &str = "photo:3f2a91c0-0000-4000-8000-0000000000aa:original";
const COVER: &str = "photo:3f2a91c0-0000-4000-8000-0000000000aa:cover";

#[test]
fn a_photo_blob_survives_the_round_trip_unchanged() {
    let key = ContentKey::generate().expect("entropy");
    let bytes = [0xABu8; 4096];
    let blob = seal_blob(&key, ORIGINAL, &bytes).expect("sealing must succeed");
    let opened = open_blob(&key, ORIGINAL, &blob).expect("opening must succeed");
    assert_eq!(opened.as_slice(), bytes.as_slice());
}

#[test]
fn an_empty_blob_round_trips_to_an_empty_blob() {
    let key = ContentKey::generate().expect("entropy");
    let blob = seal_blob(&key, ORIGINAL, b"").expect("sealing must succeed");
    assert!(
        open_blob(&key, ORIGINAL, &blob)
            .expect("opening must succeed")
            .is_empty()
    );
}

#[test]
fn a_blob_cannot_be_opened_under_another_label() {
    let key = ContentKey::generate().expect("entropy");
    let blob = seal_blob(&key, ORIGINAL, b"the picked bytes").expect("seal");
    assert!(open_blob(&key, COVER, &blob).is_err());
}

#[test]
fn a_blob_cannot_be_opened_under_another_content_key() {
    let blob = seal_blob(
        &ContentKey::generate().expect("entropy"),
        ORIGINAL,
        b"the picked bytes",
    )
    .expect("seal");
    let other = ContentKey::generate().expect("entropy");
    assert!(open_blob(&other, ORIGINAL, &blob).is_err());
}

#[test]
fn a_truncated_blob_fails_closed_rather_than_panicking() {
    let key = ContentKey::generate().expect("entropy");
    let blob = seal_blob(&key, ORIGINAL, b"the picked bytes").expect("seal");
    for cut in [0usize, 1, 12, 24, 30, 39] {
        assert!(open_blob(&key, ORIGINAL, &blob[..cut]).is_err());
    }
    assert!(open_blob(&key, ORIGINAL, &blob[..blob.len() - 1]).is_err());
}

#[test]
fn an_empty_label_is_refused() {
    let key = ContentKey::generate().expect("entropy");
    assert!(seal_blob(&key, "", b"the picked bytes").is_err());
    let blob = seal_blob(&key, ORIGINAL, b"the picked bytes").expect("seal");
    assert!(open_blob(&key, "", &blob).is_err());
}

#[test]
fn blob_length_does_not_track_plaintext_length_within_a_bucket() {
    let key = ContentKey::generate().expect("entropy");
    let empty = seal_blob(&key, ORIGINAL, b"").expect("seal").len();
    let medium = seal_blob(&key, ORIGINAL, &[b'x'; 200]).expect("seal").len();
    let long = seal_blob(&key, ORIGINAL, &[b'x'; 300]).expect("seal").len();
    assert_eq!(empty, medium);
    assert_eq!(medium, 24 + 256 + 16);
    assert_eq!(long, 24 + 512 + 16);
}
