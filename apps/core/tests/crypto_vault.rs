use leafypuff_core::domain::EntryId;
use leafypuff_core::domain::crypto::{
    ContentKey, FIELD_BODY, FieldContext, KeyVault, RecoveryCode, open, seal,
};

const PASSPHRASE: &str = "seven green rabbits sleep";
const REPLACEMENT: &str = "eight blue rabbits wake";

fn context(entry_id: EntryId) -> FieldContext<'static> {
    FieldContext {
        entry_id,
        field_name: FIELD_BODY,
        field_updated_at_ms: 1,
    }
}

fn assert_same_content_key(entry_id: EntryId, left: &ContentKey, right: &ContentKey) {
    let field = seal(left, &context(entry_id), b"the same content key or not").expect("seal");
    let opened = open(right, &context(entry_id), &field).expect("the keys must be identical");
    assert_eq!(opened.as_slice(), b"the same content key or not".as_slice());
}

#[test]
fn a_vault_unlocks_with_the_passphrase_it_was_created_from() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, created) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let unlocked = vault.unlock_with_passphrase(PASSPHRASE).expect("unlock");
    assert_same_content_key(EntryId::new(), &created, &unlocked);
}

#[test]
fn a_wrong_passphrase_fails_closed() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, _) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    assert!(
        vault
            .unlock_with_passphrase("seven green rabbits sleeps")
            .is_err()
    );
    assert!(vault.unlock_with_passphrase("").is_err());
}

#[test]
fn the_recovery_code_reconstructs_the_same_content_key() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, created) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let typed = RecoveryCode::parse(&code.to_code_string()).expect("the printed code must parse");
    let unlocked = vault
        .unlock_with_recovery_code(&typed)
        .expect("recovery unlock");
    assert_same_content_key(EntryId::new(), &created, &unlocked);
}

#[test]
fn a_wrong_recovery_code_fails_closed() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, _) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let other = RecoveryCode::generate().expect("entropy");
    assert!(vault.unlock_with_recovery_code(&other).is_err());
}

#[test]
fn changing_the_passphrase_keeps_the_content_key_and_re_encrypts_nothing() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, created) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let entry_id = EntryId::new();
    let field = seal(&created, &context(entry_id), b"written before the change").expect("seal");

    let rewrapped = vault
        .rewrap_passphrase(PASSPHRASE, REPLACEMENT)
        .expect("rewrap");

    assert_eq!(rewrapped.recovery_slot, vault.recovery_slot);
    assert_ne!(rewrapped.passphrase_slot, vault.passphrase_slot);
    assert_ne!(rewrapped.passphrase_salt, vault.passphrase_salt);

    let unlocked = rewrapped
        .unlock_with_passphrase(REPLACEMENT)
        .expect("the new passphrase must unlock");
    let opened = open(&unlocked, &context(entry_id), &field)
        .expect("an entry sealed before the change must open after it");
    assert_eq!(opened.as_slice(), b"written before the change".as_slice());
}

#[test]
fn the_old_passphrase_stops_working_after_a_rewrap() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, _) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let rewrapped = vault
        .rewrap_passphrase(PASSPHRASE, REPLACEMENT)
        .expect("rewrap");
    assert!(rewrapped.unlock_with_passphrase(PASSPHRASE).is_err());
}

#[test]
fn a_rewrap_with_the_wrong_current_passphrase_is_refused() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, _) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    assert!(
        vault
            .rewrap_passphrase("not the passphrase", REPLACEMENT)
            .is_err()
    );
}

#[test]
fn the_recovery_code_still_works_after_a_passphrase_change() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, created) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let rewrapped = vault
        .rewrap_passphrase(PASSPHRASE, REPLACEMENT)
        .expect("rewrap");
    let unlocked = rewrapped
        .unlock_with_recovery_code(&code)
        .expect("recovery survives a passphrase change");
    assert_same_content_key(EntryId::new(), &created, &unlocked);
}

#[test]
fn a_passphrase_slot_cannot_be_opened_from_the_recovery_slot() {
    let code = RecoveryCode::generate().expect("entropy");
    let (vault, _) = KeyVault::create(PASSPHRASE, &code).expect("vault creation");
    let swapped = KeyVault {
        passphrase_salt: vault.passphrase_salt,
        passphrase_slot: vault.recovery_slot.clone(),
        recovery_slot: vault.passphrase_slot.clone(),
    };
    assert!(swapped.unlock_with_passphrase(PASSPHRASE).is_err());
    assert!(swapped.unlock_with_recovery_code(&code).is_err());
}
