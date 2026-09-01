use axum::http::StatusCode;

use crate::domain::admin::AdminError;
use crate::domain::catalog::CatalogError;
use crate::domain::privacy::PrivacyError;
use crate::domain::rbac::RbacError;

use super::error::{
    ApiError, DETAIL_ACCOUNT_NOT_FOUND, DETAIL_ALREADY_FULFILLED, DETAIL_BUNDLE_NOT_FOUND,
    DETAIL_INTERNAL, DETAIL_NO_CATALOG, DETAIL_NOT_PERMITTED, DETAIL_REQUEST_NOT_FOUND,
    DETAIL_ROLE_NOT_FOUND, ERR_ACCOUNT_NOT_FOUND, ERR_ALREADY_FULFILLED, ERR_BUNDLE_NOT_FOUND,
    ERR_FORBIDDEN, ERR_INTERNAL, ERR_MALFORMED_BUNDLE, ERR_NO_CATALOG, ERR_REQUEST_NOT_FOUND,
    ERR_ROLE_NOT_FOUND,
};

impl From<RbacError> for ApiError {
    fn from(error: RbacError) -> Self {
        match error {
            RbacError::Forbidden => {
                Self::new(StatusCode::FORBIDDEN, ERR_FORBIDDEN, DETAIL_NOT_PERMITTED)
            }
            RbacError::RoleNotFound => Self::new(
                StatusCode::NOT_FOUND,
                ERR_ROLE_NOT_FOUND,
                DETAIL_ROLE_NOT_FOUND,
            ),
            RbacError::Storage(reason) => {
                tracing::error!(%reason, "an rbac request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}

impl From<AdminError> for ApiError {
    fn from(error: AdminError) -> Self {
        match error {
            AdminError::Forbidden => {
                Self::new(StatusCode::FORBIDDEN, ERR_FORBIDDEN, DETAIL_NOT_PERMITTED)
            }
            AdminError::AccountNotFound => Self::new(
                StatusCode::NOT_FOUND,
                ERR_ACCOUNT_NOT_FOUND,
                DETAIL_ACCOUNT_NOT_FOUND,
            ),
            AdminError::Storage(reason) => {
                tracing::error!(%reason, "an admin request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}

impl From<CatalogError> for ApiError {
    fn from(error: CatalogError) -> Self {
        match error {
            CatalogError::Forbidden => {
                Self::new(StatusCode::FORBIDDEN, ERR_FORBIDDEN, DETAIL_NOT_PERMITTED)
            }
            CatalogError::NotFound => Self::new(
                StatusCode::NOT_FOUND,
                ERR_BUNDLE_NOT_FOUND,
                DETAIL_BUNDLE_NOT_FOUND,
            ),
            CatalogError::NonePublished => {
                Self::new(StatusCode::NOT_FOUND, ERR_NO_CATALOG, DETAIL_NO_CATALOG)
            }
            CatalogError::Malformed(detail) => Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                ERR_MALFORMED_BUNDLE,
                &detail,
            ),
            CatalogError::Storage(reason) => {
                tracing::error!(%reason, "a catalog request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}

impl From<PrivacyError> for ApiError {
    fn from(error: PrivacyError) -> Self {
        match error {
            PrivacyError::Forbidden => {
                Self::new(StatusCode::FORBIDDEN, ERR_FORBIDDEN, DETAIL_NOT_PERMITTED)
            }
            PrivacyError::NotFound => Self::new(
                StatusCode::NOT_FOUND,
                ERR_REQUEST_NOT_FOUND,
                DETAIL_REQUEST_NOT_FOUND,
            ),
            PrivacyError::AlreadyFulfilled => Self::new(
                StatusCode::CONFLICT,
                ERR_ALREADY_FULFILLED,
                DETAIL_ALREADY_FULFILLED,
            ),
            PrivacyError::Storage(reason) => {
                tracing::error!(%reason, "a privacy request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}
