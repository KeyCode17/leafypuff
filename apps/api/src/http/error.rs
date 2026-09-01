use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use super::envelope::Envelope;

pub const ERR_DEPENDENCY_UNAVAILABLE: &str = "DEPENDENCY_UNAVAILABLE";
pub const ERR_INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
pub const ERR_EMAIL_NOT_VERIFIED: &str = "EMAIL_NOT_VERIFIED";
pub const ERR_EMAIL_TAKEN: &str = "EMAIL_ALREADY_REGISTERED";
pub const ERR_TOO_MANY_ATTEMPTS: &str = "TOO_MANY_ATTEMPTS";
pub const ERR_MAIL_UNAVAILABLE: &str = "MAIL_UNAVAILABLE";
pub const ERR_INTERNAL: &str = "INTERNAL";
pub const ERR_FORBIDDEN: &str = "FORBIDDEN";
pub const ERR_MALFORMED_BATCH: &str = "MALFORMED_BATCH";
pub const ERR_OBJECT_NOT_FOUND: &str = "OBJECT_NOT_FOUND";
pub const ERR_OBJECT_TOO_LARGE: &str = "OBJECT_TOO_LARGE";
pub const ERR_ROLE_NOT_FOUND: &str = "ROLE_NOT_FOUND";
pub const ERR_ACCOUNT_NOT_FOUND: &str = "ACCOUNT_NOT_FOUND";
pub const ERR_BUNDLE_NOT_FOUND: &str = "BUNDLE_NOT_FOUND";
pub const ERR_NO_CATALOG: &str = "NO_CATALOG_PUBLISHED";
pub const ERR_MALFORMED_BUNDLE: &str = "MALFORMED_BUNDLE";
pub const ERR_REQUEST_NOT_FOUND: &str = "DATA_REQUEST_NOT_FOUND";
pub const ERR_ALREADY_FULFILLED: &str = "ALREADY_FULFILLED";

const MESSAGE_FAILED: &str = "Request failed";
pub(super) const DETAIL_INVALID_CREDENTIALS: &str = "Invalid credentials";
pub(super) const DETAIL_EMAIL_NOT_VERIFIED: &str = "Email is not verified";
pub(super) const DETAIL_EMAIL_TAKEN: &str = "Email already registered";
pub(super) const DETAIL_TOO_MANY_ATTEMPTS: &str = "Too many attempts";
pub(super) const DETAIL_MAIL_UNAVAILABLE: &str = "The mail provider is unavailable";
pub(super) const DETAIL_INTERNAL: &str = "Something went wrong";
pub(super) const DETAIL_FORBIDDEN: &str = "That entry belongs to another account";
pub(super) const DETAIL_OBJECT_NOT_FOUND: &str = "No such object";
pub(super) const DETAIL_OBJECT_TOO_LARGE: &str = "Object is larger than the ceiling";
pub(super) const DETAIL_NOT_PERMITTED: &str = "You do not have permission to do that";
pub(super) const DETAIL_ROLE_NOT_FOUND: &str = "No such role";
pub(super) const DETAIL_ACCOUNT_NOT_FOUND: &str = "No such account";
pub(super) const DETAIL_BUNDLE_NOT_FOUND: &str = "No such bundle";
pub(super) const DETAIL_NO_CATALOG: &str = "No catalog has been published";
pub(super) const DETAIL_REQUEST_NOT_FOUND: &str = "No such data request";
pub(super) const DETAIL_ALREADY_FULFILLED: &str = "That request was already fulfilled";

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    detail: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, detail: &str) -> Self {
        Self {
            status,
            code,
            detail: detail.to_owned(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Envelope::failed(MESSAGE_FAILED, self.code, &self.detail);
        (self.status, Json(body)).into_response()
    }
}
