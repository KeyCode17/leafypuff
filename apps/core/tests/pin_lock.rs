use leafypuff_core::domain::crypto::{hash_pin, verify_pin};

const PIN: &str = "4821";

#[test]
fn a_pin_is_stored_as_a_hash_and_never_as_its_digits() {
    let stored = hash_pin(PIN).expect("the pin hashes");

    assert!(!stored.contains(PIN));
    assert!(stored.starts_with("$argon2id$"));
    assert!(verify_pin(PIN, &stored));
}

#[test]
fn a_wrong_pin_does_not_verify() {
    let stored = hash_pin(PIN).expect("the pin hashes");

    assert!(!verify_pin("0000", &stored));
    assert!(!verify_pin("", &stored));
}

#[test]
fn two_hashes_of_one_pin_differ_because_the_salt_does() {
    let first = hash_pin(PIN).expect("the pin hashes");
    let second = hash_pin(PIN).expect("the pin hashes");

    assert_ne!(first, second);
    assert!(verify_pin(PIN, &first));
    assert!(verify_pin(PIN, &second));
}

#[test]
fn a_malformed_stored_value_verifies_as_false_rather_than_panicking() {
    assert!(!verify_pin(PIN, "not-a-phc-string"));
    assert!(!verify_pin(PIN, ""));
}
