use std::collections::HashMap;

use leafypuff_api::infrastructure::{Config, ConfigError};

fn source(map: HashMap<String, String>) -> impl Fn(&str) -> Option<String> {
    move |key: &str| map.get(key).cloned()
}

fn rejection(map: HashMap<String, String>, reason: &str) -> ConfigError {
    match Config::from_env(&source(map)) {
        Ok(_) => panic!("{reason}"),
        Err(error) => error,
    }
}

fn complete() -> HashMap<String, String> {
    let pairs = [
        ("DATABASE_URL", "postgres:///leafypuff".to_owned()),
        ("S3_ENDPOINT", "127.0.0.1:3900".to_owned()),
        ("S3_BUCKET", "leafypuff".to_owned()),
        ("S3_ACCESS_KEY", "access".to_owned()),
        ("S3_SECRET_KEY", "secret".to_owned()),
        ("RESEND_API_KEY", "re_test".to_owned()),
        ("JWT_SIGNING_SECRET", "a".repeat(32)),
        ("MAIL_FROM", "leafyPuff <no-reply@example.test>".to_owned()),
        ("OTP_PEPPER", "0".repeat(64)),
        ("PORT", "8080".to_owned()),
    ];
    pairs
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect()
}

#[test]
fn a_missing_resend_key_is_named_in_the_error() {
    let mut map = complete();
    map.remove("RESEND_API_KEY");
    let error = rejection(map, "a missing resend key must fail");
    assert!(matches!(error, ConfigError::Missing(ref key) if key == "RESEND_API_KEY"));
}

#[test]
fn a_short_pepper_is_rejected_by_name() {
    let mut map = complete();
    map.insert("OTP_PEPPER".to_owned(), "abc".to_owned());
    let error = rejection(map, "a short pepper must fail");
    assert!(matches!(error, ConfigError::Invalid(ref key, _) if key == "OTP_PEPPER"));
}

#[test]
fn a_short_signing_secret_is_rejected_by_name() {
    let mut map = complete();
    map.insert("JWT_SIGNING_SECRET".to_owned(), "tooshort".to_owned());
    let error = rejection(map, "a short signing secret must fail");
    assert!(matches!(error, ConfigError::Invalid(ref key, _) if key == "JWT_SIGNING_SECRET"));
}

#[test]
fn an_uppercase_pepper_is_rejected() {
    let mut map = complete();
    map.insert("OTP_PEPPER".to_owned(), "A".repeat(64));
    let error = rejection(map, "an uppercase pepper must fail");
    assert!(matches!(error, ConfigError::Invalid(ref key, _) if key == "OTP_PEPPER"));
}

#[test]
fn a_complete_environment_parses() {
    let Ok(config) = Config::from_env(&source(complete())) else {
        panic!("a complete environment must parse")
    };
    assert_eq!(config.port, 8080);
    assert_eq!(config.s3_bucket, "leafypuff");
    assert_eq!(config.otp_pepper, [0u8; 32]);
}

#[test]
fn a_from_header_with_no_address_is_refused_at_startup() {
    // systemd splits an unquoted Environment= value on whitespace, which turned
    // "leafyPuff <no-reply@example.test>" into "leafyPuff" on the way into the service. The
    // provider would have rejected every send, a long way from the deploy that caused it.
    let mut map = complete();
    map.insert("MAIL_FROM".to_owned(), "leafyPuff".to_owned());

    let error = rejection(map, "a from header with no address must fail");

    assert!(matches!(error, ConfigError::Invalid(ref key, _) if key == "MAIL_FROM"));
}
