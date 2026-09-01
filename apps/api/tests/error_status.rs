use axum::http::StatusCode;
use axum::response::IntoResponse;
use leafypuff_api::domain::iam::IamError;
use leafypuff_api::http::error::ApiError;

/// A CDN in front of this replaces the body of a 502 with its own error page, so the envelope
/// never reaches the device and the client sees unparseable text instead of MAIL_UNAVAILABLE.
/// That is exactly what happened in production: Resend answered 401, the API answered 502, and
/// the phone reported "No connection" while its network was fine.
#[test]
fn a_mail_failure_answers_503_so_the_envelope_survives_a_proxy() {
    let response = ApiError::from(IamError::Mail("provider refused".to_owned())).into_response();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_ne!(response.status(), StatusCode::BAD_GATEWAY);
}

#[test]
fn the_statuses_a_proxy_rewrites_are_not_used_for_any_iam_failure() {
    let rewritten = [StatusCode::BAD_GATEWAY, StatusCode::GATEWAY_TIMEOUT];
    let failures = [
        IamError::InvalidCredentials,
        IamError::EmailAlreadyRegistered,
        IamError::InvalidCode,
        IamError::EmailNotVerified,
        IamError::ChallengeUnusable,
        IamError::TooManyAttempts,
        IamError::Mail("provider refused".to_owned()),
        IamError::Storage("database refused".to_owned()),
    ];

    for failure in failures {
        let status = ApiError::from(failure).into_response().status();
        assert!(
            !rewritten.contains(&status),
            "a proxy would replace the body of {status}"
        );
    }
}
