use leafypuff_api::domain::media::{ObjectKey, Variant};
use uuid::Uuid;

#[test]
fn a_key_is_built_only_from_uuids_and_a_known_variant() {
    let account_id = Uuid::from_u128(1);
    let photo_id = Uuid::from_u128(2);

    let rendered = ObjectKey::new(account_id, photo_id, Variant::Original).to_string();

    assert_eq!(
        rendered,
        "accounts/00000000000000000000000000000001/photos/00000000000000000000000000000002/original"
    );
    assert!(!rendered.contains(".."));
    assert!(!rendered.contains("//"));
}

#[test]
fn every_variant_renders_a_distinct_key() {
    let account_id = Uuid::new_v4();
    let photo_id = Uuid::new_v4();

    let rendered: Vec<String> = Variant::ALL
        .into_iter()
        .map(|variant| ObjectKey::new(account_id, photo_id, variant).to_string())
        .collect();

    assert_eq!(rendered.len(), 2);
    assert_ne!(rendered[0], rendered[1]);
}

#[test]
fn an_unknown_variant_string_does_not_parse() {
    assert!(Variant::parse("../../etc/passwd").is_none());
    assert!(Variant::parse("thumbnail").is_none());
    assert_eq!(Variant::parse("original"), Some(Variant::Original));
}
