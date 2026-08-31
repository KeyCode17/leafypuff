use api_testing::World;
use chrono::Duration;
use leafypuff_api::application::iam::Session;
use leafypuff_api::application::iam::{
    CompleteSignInInput, RefreshInput, RegisterInput, StartSignInInput, VerifyEmailInput,
};
use leafypuff_api::domain::iam::policy::OTP_TTL_SECONDS;
use leafypuff_api::domain::iam::{IamError, OtpCode, OtpPurpose};

const EMAIL: &str = "Person@Example.test";
const NORMALISED: &str = "person@example.test";
const PASSWORD: &str = "correct horse battery";
const DEVICE: &str = "pixel-8";

fn refused(outcome: Result<Session, IamError>, reason: &str) -> IamError {
    match outcome {
        Ok(_) => panic!("{reason}"),
        Err(error) => error,
    }
}

fn registration() -> RegisterInput {
    RegisterInput {
        email: EMAIL.to_owned(),
        password: PASSWORD.to_owned(),
        display_name: None,
    }
}

async fn verified_world() -> World {
    let world = World::default();
    world.generator.queue("123456");
    world
        .register()
        .execute(registration())
        .await
        .expect("registration succeeds");
    world
        .verify_email()
        .execute(VerifyEmailInput {
            email: EMAIL.to_owned(),
            code: "123456".to_owned(),
        })
        .await
        .expect("verification succeeds");
    world
}

#[tokio::test]
async fn registration_normalises_the_address_and_mails_a_code() {
    let world = World::default();
    world.generator.queue("123456");

    world
        .register()
        .execute(registration())
        .await
        .expect("registration succeeds");

    let stored = world.accounts.snapshot();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].email, NORMALISED);
    assert!(!stored[0].is_verified());
    assert_ne!(stored[0].password_hash, PASSWORD);
    assert_eq!(
        world.mailer.sent(),
        vec![(
            NORMALISED.to_owned(),
            OtpPurpose::VerifyEmail,
            "123456".to_owned()
        )]
    );
}

#[tokio::test]
async fn registering_again_before_verifying_mails_a_fresh_code() {
    let world = World::default();
    world.generator.queue("123456");
    world.generator.queue("654321");
    world
        .register()
        .execute(registration())
        .await
        .expect("the first registration succeeds");

    world
        .register()
        .execute(registration())
        .await
        .expect("an unverified address may ask for another code");

    assert_eq!(world.accounts.snapshot().len(), 1);
    assert_eq!(world.mailer.sent().len(), 2);
    assert_eq!(world.mailer.last_code(), "654321");
}

#[tokio::test]
async fn registering_a_verified_address_is_rejected_and_mails_nothing() {
    let world = verified_world().await;

    let conflict = world
        .register()
        .execute(registration())
        .await
        .expect_err("a verified address must be rejected");

    assert!(matches!(conflict, IamError::EmailAlreadyRegistered));
    assert_eq!(world.mailer.sent().len(), 1);
}

#[tokio::test]
async fn the_attempt_ceiling_closes_the_challenge() {
    let world = World::default();
    world.generator.queue("123456");
    world
        .register()
        .execute(registration())
        .await
        .expect("registration succeeds");

    for _ in 1..OtpCode::MAX_ATTEMPTS {
        let refused = world
            .verify_email()
            .execute(VerifyEmailInput {
                email: EMAIL.to_owned(),
                code: "000000".to_owned(),
            })
            .await
            .expect_err("a wrong code must be refused");
        assert!(matches!(refused, IamError::InvalidCode));
    }

    let exhausted = world
        .verify_email()
        .execute(VerifyEmailInput {
            email: EMAIL.to_owned(),
            code: "000000".to_owned(),
        })
        .await
        .expect_err("the ceiling must be reported");
    assert!(matches!(exhausted, IamError::TooManyAttempts));

    let closed = world
        .verify_email()
        .execute(VerifyEmailInput {
            email: EMAIL.to_owned(),
            code: "123456".to_owned(),
        })
        .await
        .expect_err("the challenge is closed even for the right code");
    assert!(matches!(closed, IamError::ChallengeUnusable));
}

#[tokio::test]
async fn a_verification_code_expires() {
    let world = World::default();
    world.generator.queue("123456");
    world
        .register()
        .execute(registration())
        .await
        .expect("registration succeeds");
    world.clock.advance(Duration::seconds(OTP_TTL_SECONDS));

    let rejected = world
        .verify_email()
        .execute(VerifyEmailInput {
            email: EMAIL.to_owned(),
            code: "123456".to_owned(),
        })
        .await
        .expect_err("an expired code must be refused");

    assert!(matches!(rejected, IamError::ChallengeUnusable));
    assert!(!world.accounts.snapshot()[0].is_verified());
}

#[tokio::test]
async fn a_verification_code_does_not_work_twice() {
    let world = verified_world().await;

    let replayed = world
        .verify_email()
        .execute(VerifyEmailInput {
            email: EMAIL.to_owned(),
            code: "123456".to_owned(),
        })
        .await
        .expect_err("a consumed code must be refused");

    assert!(matches!(replayed, IamError::ChallengeUnusable));
}

#[tokio::test]
async fn an_unknown_address_costs_the_same_verification_as_a_wrong_password() {
    let world = verified_world().await;
    let before = world.hasher.verifications();

    let unknown = world
        .start_sign_in()
        .execute(StartSignInInput {
            email: "nobody@example.test".to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect_err("an unknown address must be refused");
    let wrong = world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: "not the password".to_owned(),
        })
        .await
        .expect_err("a wrong password must be refused");

    assert!(matches!(unknown, IamError::InvalidCredentials));
    assert!(matches!(wrong, IamError::InvalidCredentials));
    assert_eq!(world.hasher.verifications(), before + 2);
    assert_eq!(world.mailer.sent().len(), 1);
}

#[tokio::test]
async fn signing_in_needs_a_verified_address() {
    let world = World::default();
    world.generator.queue("123456");
    world
        .register()
        .execute(registration())
        .await
        .expect("registration succeeds");

    let rejected = world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect_err("an unverified address must be refused");

    assert!(matches!(rejected, IamError::EmailNotVerified));
}

#[tokio::test]
async fn a_wrong_code_is_counted_and_the_right_one_still_works() {
    let world = verified_world().await;
    world.generator.queue("654321");
    world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect("the password check passes");

    let wrong = refused(
        world
            .complete_sign_in()
            .execute(CompleteSignInInput {
                email: EMAIL.to_owned(),
                code: "000000".to_owned(),
                device_id: DEVICE.to_owned(),
            })
            .await,
        "a wrong code must be refused",
    );
    assert!(matches!(wrong, IamError::InvalidCode));

    let session = world
        .complete_sign_in()
        .execute(CompleteSignInInput {
            email: EMAIL.to_owned(),
            code: world.mailer.last_code(),
            device_id: DEVICE.to_owned(),
        })
        .await
        .expect("the mailed code must be accepted");
    assert!(session.access_token.starts_with("access:"));
}

#[tokio::test]
async fn refreshing_rotates_the_credential_and_the_old_secret_dies() {
    let world = verified_world().await;
    world.generator.queue("654321");
    world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect("the password check passes");
    let first = world
        .complete_sign_in()
        .execute(CompleteSignInInput {
            email: EMAIL.to_owned(),
            code: world.mailer.last_code(),
            device_id: DEVICE.to_owned(),
        })
        .await
        .expect("the sign-in completes");

    let second = world
        .refresh()
        .execute(RefreshInput {
            refresh_secret: first.refresh_secret.clone(),
            device_id: DEVICE.to_owned(),
        })
        .await
        .expect("the rotation succeeds");
    assert_ne!(second.refresh_secret, first.refresh_secret);

    let replayed = refused(
        world
            .refresh()
            .execute(RefreshInput {
                refresh_secret: first.refresh_secret,
                device_id: DEVICE.to_owned(),
            })
            .await,
        "the replaced secret must be dead",
    );
    assert!(matches!(replayed, IamError::InvalidCredentials));
}

#[tokio::test]
async fn a_credential_from_another_device_is_refused() {
    let world = verified_world().await;
    world.generator.queue("654321");
    world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect("the password check passes");
    let session = world
        .complete_sign_in()
        .execute(CompleteSignInInput {
            email: EMAIL.to_owned(),
            code: world.mailer.last_code(),
            device_id: DEVICE.to_owned(),
        })
        .await
        .expect("the sign-in completes");

    let rejected = refused(
        world
            .refresh()
            .execute(RefreshInput {
                refresh_secret: session.refresh_secret,
                device_id: "someone-elses-laptop".to_owned(),
            })
            .await,
        "a credential presented from another device must be refused",
    );

    assert!(matches!(rejected, IamError::InvalidCredentials));
}
