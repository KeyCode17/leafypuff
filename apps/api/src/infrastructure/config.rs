#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    Missing(String),
    #[error("Invalid value for {0}: {1}")]
    Invalid(String, String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub port: u16,
}

impl Config {
    pub fn from_env(source: &dyn Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let required = |key: &str| source(key).ok_or_else(|| ConfigError::Missing(key.to_owned()));
        let port_raw = source("PORT").unwrap_or_else(|| "8080".to_owned());
        let port = port_raw
            .parse()
            .map_err(|_| ConfigError::Invalid("PORT".to_owned(), port_raw))?;

        Ok(Self {
            database_url: required("DATABASE_URL")?,
            s3_endpoint: required("S3_ENDPOINT")?,
            s3_bucket: required("S3_BUCKET")?,
            s3_access_key: required("S3_ACCESS_KEY")?,
            s3_secret_key: required("S3_SECRET_KEY")?,
            port,
        })
    }
}
