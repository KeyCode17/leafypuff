use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::TryRng;
use rand::rngs::SysRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::iam::error::ERR_ENTROPY_UNAVAILABLE;
use crate::domain::iam::policy::ACCESS_TOKEN_TTL_SECONDS;
use crate::domain::iam::{IamError, TokenIssuer, TokenVerifier};

const ISSUER: &str = "leafypuff-api";
const REFRESH_SECRET_BYTES: usize = 32;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
}

pub struct JwtTokenIssuer {
    key: EncodingKey,
    verifying_key: DecodingKey,
}

impl JwtTokenIssuer {
    pub fn new(signing_secret: &str) -> Self {
        Self {
            key: EncodingKey::from_secret(signing_secret.as_bytes()),
            verifying_key: DecodingKey::from_secret(signing_secret.as_bytes()),
        }
    }
}

impl TokenIssuer for JwtTokenIssuer {
    fn access_token(&self, account_id: Uuid) -> Result<String, IamError> {
        let issued_at = Utc::now().timestamp();
        let claims = Claims {
            sub: account_id.to_string(),
            iss: ISSUER.to_owned(),
            iat: issued_at,
            exp: issued_at + ACCESS_TOKEN_TTL_SECONDS,
        };
        encode(&Header::new(Algorithm::HS256), &claims, &self.key)
            .map_err(|error| IamError::Storage(error.to_string()))
    }

    fn refresh_secret(&self) -> Result<String, IamError> {
        let mut bytes = [0u8; REFRESH_SECRET_BYTES];
        SysRng
            .try_fill_bytes(&mut bytes)
            .map_err(|_| IamError::Storage(ERR_ENTROPY_UNAVAILABLE.to_owned()))?;
        Ok(data_encoding::BASE64URL_NOPAD.encode(&bytes))
    }

    fn digest(&self, secret: &str) -> String {
        blake3::hash(secret.as_bytes()).to_hex().to_string()
    }
}

impl TokenVerifier for JwtTokenIssuer {
    fn account_id(&self, access_token: &str) -> Result<Uuid, IamError> {
        let mut rules = Validation::new(Algorithm::HS256);
        rules.set_issuer(&[ISSUER]);
        let claims = decode::<Claims>(access_token, &self.verifying_key, &rules)
            .map_err(|_| IamError::InvalidCredentials)?
            .claims;
        Uuid::parse_str(&claims.sub).map_err(|_| IamError::InvalidCredentials)
    }
}
