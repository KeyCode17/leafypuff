use leafypuff_core::domain::EntryId;
use leafypuff_core::domain::crypto::{ContentKey, FIELD_BODY, FieldContext, open, seal};
use proptest::prelude::{ProptestConfig, any, prop_assert, prop_assert_eq, proptest};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn arbitrary_bytes_survive_the_round_trip(plaintext in any::<Vec<u8>>()) {
        let key = ContentKey::generate().expect("entropy");
        let entry_id = EntryId::new();
        let context = FieldContext { entry_id, field_name: FIELD_BODY, field_updated_at_ms: 7 };
        let field = seal(&key, &context, &plaintext).expect("seal");
        let opened = open(&key, &context, &field).expect("open");
        prop_assert_eq!(opened, plaintext);
    }

    #[test]
    fn arbitrary_text_survives_the_round_trip(text in ".*") {
        let key = ContentKey::generate().expect("entropy");
        let entry_id = EntryId::new();
        let context = FieldContext { entry_id, field_name: FIELD_BODY, field_updated_at_ms: 7 };
        let field = seal(&key, &context, text.as_bytes()).expect("seal");
        let opened = open(&key, &context, &field).expect("open");
        prop_assert_eq!(String::from_utf8(opened).expect("utf8"), text);
    }

    #[test]
    fn ciphertext_length_reveals_only_the_bucket(plaintext in any::<Vec<u8>>()) {
        let key = ContentKey::generate().expect("entropy");
        let context = FieldContext { entry_id: EntryId::new(), field_name: FIELD_BODY, field_updated_at_ms: 7 };
        let field = seal(&key, &context, &plaintext).expect("seal");
        prop_assert!((field.ciphertext.len() - 16).is_multiple_of(256));
        prop_assert!(field.ciphertext.len() - 16 >= plaintext.len() + 4);
    }
}
