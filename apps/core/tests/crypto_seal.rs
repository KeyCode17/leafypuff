use leafypuff_core::domain::EntryId;
use leafypuff_core::domain::crypto::{
    ContentKey, FIELD_BODY, FIELD_TITLE, FieldContext, SealedField, open, seal,
};

fn context(entry_id: EntryId, field_name: &str, field_updated_at_ms: i64) -> FieldContext<'_> {
    FieldContext {
        entry_id,
        field_name,
        field_updated_at_ms,
    }
}

fn sealed(key: &ContentKey, entry_id: EntryId, body: &[u8]) -> SealedField {
    seal(key, &context(entry_id, FIELD_BODY, 1), body).expect("sealing must succeed")
}

#[test]
fn a_body_survives_the_round_trip_unchanged() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let body = "Tea on the balcony, and the rabbit slept through all of it.".as_bytes();
    let opened = open(
        &key,
        &context(entry_id, FIELD_BODY, 1),
        &sealed(&key, entry_id, body),
    )
    .expect("opening must succeed");
    assert_eq!(opened.as_slice(), body);
}

#[test]
fn an_empty_body_round_trips_to_an_empty_body() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let opened = open(
        &key,
        &context(entry_id, FIELD_BODY, 1),
        &sealed(&key, entry_id, b""),
    )
    .expect("opening must succeed");
    assert!(opened.is_empty());
}

#[test]
fn multi_byte_utf8_round_trips_byte_for_byte() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let body = "🐰 こんにちは — café, naïve, Ω≈ç√".as_bytes();
    let opened = open(
        &key,
        &context(entry_id, FIELD_BODY, 1),
        &sealed(&key, entry_id, body),
    )
    .expect("opening must succeed");
    assert_eq!(
        String::from_utf8(opened).expect("utf8"),
        "🐰 こんにちは — café, naïve, Ω≈ç√"
    );
}

#[test]
fn a_body_cannot_be_opened_under_another_entry_id() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let field = sealed(&key, entry_id, b"a private evening");
    let result = open(&key, &context(EntryId::new(), FIELD_BODY, 1), &field);
    assert!(result.is_err());
}

#[test]
fn a_body_cannot_be_opened_as_a_title() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let field = sealed(&key, entry_id, b"a private evening");
    assert!(open(&key, &context(entry_id, FIELD_TITLE, 1), &field).is_err());
}

#[test]
fn a_body_cannot_be_opened_at_another_revision() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let field = sealed(&key, entry_id, b"a private evening");
    assert!(open(&key, &context(entry_id, FIELD_BODY, 2), &field).is_err());
}

#[test]
fn a_body_cannot_be_opened_under_another_content_key() {
    let entry_id = EntryId::new();
    let field = sealed(
        &ContentKey::generate().expect("entropy"),
        entry_id,
        b"a private evening",
    );
    let other = ContentKey::generate().expect("entropy");
    assert!(open(&other, &context(entry_id, FIELD_BODY, 1), &field).is_err());
}

#[test]
fn ciphertext_length_does_not_track_plaintext_length_within_a_bucket() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let empty = sealed(&key, entry_id, b"").ciphertext.len();
    let short = sealed(&key, entry_id, &[b'x'; 10]).ciphertext.len();
    let medium = sealed(&key, entry_id, &[b'x'; 200]).ciphertext.len();
    let long = sealed(&key, entry_id, &[b'x'; 300]).ciphertext.len();
    assert_eq!(empty, short);
    assert_eq!(short, medium);
    assert_eq!(medium, 256 + 16);
    assert_eq!(long, 512 + 16);
}

#[test]
fn two_seals_of_the_same_body_differ() {
    let key = ContentKey::generate().expect("entropy");
    let entry_id = EntryId::new();
    let first = sealed(&key, entry_id, b"same body");
    let second = sealed(&key, entry_id, b"same body");
    assert_ne!(first.nonce, second.nonce);
    assert_ne!(first.ciphertext, second.ciphertext);
}
