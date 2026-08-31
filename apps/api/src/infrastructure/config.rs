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
    pub minio_endpoint: String,
    pub minio_bucket: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
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
            minio_endpoint: required("MINIO_ENDPOINT")?,
            minio_bucket: required("MINIO_BUCKET")?,
            minio_access_key: required("MINIO_ACCESS_KEY")?,
            minio_secret_key: required("MINIO_SECRET_KEY")?,
            port,
        })
    }
}
