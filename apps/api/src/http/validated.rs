use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use serde::de::DeserializeOwned;

use super::error::ApiError;

const ERR_MALFORMED_BODY: &str = "Request body is not valid JSON";
const ERR_VALIDATION_FAILED: &str = "Request failed validation";
const ERR_UNSUPPORTED_MEDIA: &str = "Request body must be application/json";

pub trait ValidatedBody {
    fn validate(&self) -> Result<(), &'static str>;
}

pub struct Validated<T>(pub T);

impl<S, T> FromRequest<S> for Validated<T>
where
    S: Send + Sync,
    T: DeserializeOwned + ValidatedBody,
{
    type Rejection = ApiError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<T>::from_request(request, state)
            .await
            .map_err(refuse)?;
        body.validate()
            .map_err(|_| ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, ERR_VALIDATION_FAILED))?;
        Ok(Self(body))
    }
}

fn refuse(rejection: JsonRejection) -> ApiError {
    match rejection {
        JsonRejection::JsonDataError(_) => {
            ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, ERR_VALIDATION_FAILED)
        }
        JsonRejection::MissingJsonContentType(_) => {
            ApiError::new(StatusCode::UNSUPPORTED_MEDIA_TYPE, ERR_UNSUPPORTED_MEDIA)
        }
        _ => ApiError::new(StatusCode::BAD_REQUEST, ERR_MALFORMED_BODY),
    }
}
