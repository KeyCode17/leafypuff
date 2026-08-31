use leafypuff_api::domain::sync::{EncryptedField, FieldEnvelope, resolve_field};
use uuid::Uuid;

fn envelope(ciphertext: &[u8], updated_at_ms: i64, device_id: Uuid) -> FieldEnvelope {
    FieldEnvelope {
        ciphertext: ciphertext.to_vec(),
        nonce: vec![0_u8; 24],
        updated_at_ms,
        device_id,
    }
}

#[test]
fn a_title_from_one_device_and_a_body_from_another_both_survive() {
    let entry_id = Uuid::new_v4();
    let device_a = Uuid::new_v4();
    let device_b = Uuid::new_v4();

    let held_title = envelope(b"A-title-late", 2_000, device_a);
    let held_body = envelope(b"A-body-early", 1_000, device_a);
    let pushed_title = envelope(b"B-title-early", 1_000, device_b);
    let pushed_body = envelope(b"B-body-late", 2_000, device_b);

    let title = resolve_field(
        entry_id,
        EncryptedField::Title,
        Some(held_title.clone()),
        pushed_title,
    );
    let body = resolve_field(
        entry_id,
        EncryptedField::Body,
        Some(held_body),
        pushed_body.clone(),
    );

    assert_eq!(title.winner, held_title);
    assert_eq!(body.winner, pushed_body);

    let title_conflict = title
        .conflict
        .expect("the losing title must be fingerprinted");
    assert_eq!(title_conflict.loser_device_id, device_b);
    assert_eq!(
        title_conflict.loser_byte_len,
        i64::try_from(b"B-title-early".len()).expect("the length fits")
    );
    assert_eq!(title_conflict.loser_updated_at_ms, 1_000);
    assert_eq!(title_conflict.winner_updated_at_ms, 2_000);
    assert_eq!(title_conflict.loser_ciphertext_hash.len(), 64);

    let body_conflict = body
        .conflict
        .expect("the losing body must be fingerprinted");
    assert_eq!(body_conflict.loser_device_id, device_a);
    assert_eq!(
        body_conflict.loser_byte_len,
        i64::try_from(b"A-body-early".len()).expect("the length fits")
    );
}

#[test]
fn a_first_write_wins_without_recording_a_conflict() {
    let outcome = resolve_field(
        Uuid::new_v4(),
        EncryptedField::Title,
        None,
        envelope(b"first", 1_000, Uuid::new_v4()),
    );

    assert!(outcome.conflict.is_none());
}

#[test]
fn an_identical_replay_records_no_conflict() {
    let device = Uuid::new_v4();
    let held = envelope(b"same", 1_000, device);

    let outcome = resolve_field(
        Uuid::new_v4(),
        EncryptedField::Body,
        Some(held.clone()),
        held,
    );

    assert!(outcome.conflict.is_none());
}

#[test]
fn a_tie_is_broken_deterministically_by_device_id() {
    let entry_id = Uuid::new_v4();
    let low = Uuid::from_u128(1);
    let high = Uuid::from_u128(2);
    let held = envelope(b"held", 1_000, low);
    let incoming = envelope(b"incoming", 1_000, high);

    let first = resolve_field(
        entry_id,
        EncryptedField::Title,
        Some(held.clone()),
        incoming.clone(),
    );
    let second = resolve_field(entry_id, EncryptedField::Title, Some(held), incoming);

    assert_eq!(first.winner, second.winner);
    assert_eq!(first.winner.device_id, high);
}

#[test]
fn the_envelope_debug_prints_a_length_and_never_the_bytes() {
    let rendered = format!("{:?}", envelope(&[0xAB_u8; 64], 1_000, Uuid::from_u128(7)));

    assert!(rendered.contains("ciphertext_len: 64"));
    assert!(!rendered.contains("171"));
    assert!(!rendered.contains("ab"));
}
