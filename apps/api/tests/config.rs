use leafypuff_api::infrastructure::config::{Config, ConfigError};
use std::collections::HashMap;

fn source(map: HashMap<String, String>) -> impl Fn(&str) -> Option<String> {
    move |key: &str| map.get(key).cloned()
}

#[test]
fn a_missing_variable_is_named_in_the_error() {
    let result = Config::from_env(&source(HashMap::new()));
    let error = result.expect_err("an empty environment must fail");
    assert!(matches!(error, ConfigError::Missing(ref key) if key == "DATABASE_URL"));
}

#[test]
fn a_complete_environment_parses() {
    let mut map = HashMap::new();
    map.insert(
        "DATABASE_URL".to_owned(),
        "postgres:///leafypuff".to_owned(),
    );
    map.insert("S3_ENDPOINT".to_owned(), "127.0.0.1:9000".to_owned());
    map.insert("S3_BUCKET".to_owned(), "leafypuff".to_owned());
    map.insert("S3_ACCESS_KEY".to_owned(), "access".to_owned());
    map.insert("S3_SECRET_KEY".to_owned(), "secret".to_owned());
    map.insert("PORT".to_owned(), "8080".to_owned());

    let config = Config::from_env(&source(map)).expect("a complete environment must parse");
    assert_eq!(config.port, 8080);
    assert_eq!(config.s3_bucket, "leafypuff");
}

#[test]
fn a_non_numeric_port_is_rejected_by_name() {
    let mut map = HashMap::new();
    map.insert(
        "DATABASE_URL".to_owned(),
        "postgres:///leafypuff".to_owned(),
    );
    map.insert("S3_ENDPOINT".to_owned(), "127.0.0.1:9000".to_owned());
    map.insert("S3_BUCKET".to_owned(), "leafypuff".to_owned());
    map.insert("S3_ACCESS_KEY".to_owned(), "access".to_owned());
    map.insert("S3_SECRET_KEY".to_owned(), "secret".to_owned());
    map.insert("PORT".to_owned(), "eighty".to_owned());

    let error = Config::from_env(&source(map)).expect_err("a non-numeric port must fail");
    assert!(matches!(error, ConfigError::Invalid(ref key, _) if key == "PORT"));
}
