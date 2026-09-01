use data_encoding::HEXLOWER;

const MINIMUM_SIGNING_SECRET_BYTES: usize = 32;
const PEPPER_BYTES: usize = 32;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    Missing(String),
    #[error("Invalid value for {0}: {1}")]
    Invalid(String, String),
}

fn sender(raw: &str) -> Result<String, ConfigError> {
    if raw.contains('@') {
        return Ok(raw.to_owned());
    }
    Err(ConfigError::Invalid(
        "MAIL_FROM".to_owned(),
        "carries no address".to_owned(),
    ))
}

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub resend_api_key: String,
    pub jwt_signing_secret: String,
    pub mail_from: String,
    pub otp_pepper: [u8; PEPPER_BYTES],
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
            resend_api_key: required("RESEND_API_KEY")?,
            jwt_signing_secret: signing_secret(required("JWT_SIGNING_SECRET")?)?,
            mail_from: sender(&required("MAIL_FROM")?)?,
            otp_pepper: pepper(&required("OTP_PEPPER")?)?,
            port,
        })
    }
}

fn signing_secret(raw: String) -> Result<String, ConfigError> {
    if raw.len() < MINIMUM_SIGNING_SECRET_BYTES {
        return Err(ConfigError::Invalid(
            "JWT_SIGNING_SECRET".to_owned(),
            format!("expected at least {MINIMUM_SIGNING_SECRET_BYTES} bytes"),
        ));
    }
    Ok(raw)
}

fn pepper(raw: &str) -> Result<[u8; PEPPER_BYTES], ConfigError> {
    let invalid = || {
        ConfigError::Invalid(
            "OTP_PEPPER".to_owned(),
            format!(
                "expected {} lowercase hexadecimal characters",
                PEPPER_BYTES * 2
            ),
        )
    };
    HEXLOWER
        .decode(raw.as_bytes())
        .map_err(|_| invalid())?
        .try_into()
        .map_err(|_| invalid())
}
