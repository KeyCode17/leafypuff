use api_testing::World;
use chrono::Duration;
use leafypuff_api::application::iam::Session;
use leafypuff_api::application::iam::{
    CompleteSignInInput, ConfirmEmailChangeInput, RefreshInput, RegisterInput, ResetPasswordInput,
    StartEmailChangeInput, StartPasswordResetInput, StartSignInInput, VerifyEmailInput,
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
async fn registering_a_verified_address_mails_the_owner_a_notice_and_no_code() {
    let world = verified_world().await;

    world
        .register()
        .execute(registration())
        .await
        .expect("a verified address is answered without disclosing that it is taken");

    assert_eq!(world.mailer.sent().len(), 1);
    assert_eq!(world.mailer.notices(), vec![NORMALISED.to_owned()]);
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

const NEW_PASSWORD: &str = "a different passphrase entirely";

fn forgetting() -> StartPasswordResetInput {
    StartPasswordResetInput {
        email: EMAIL.to_owned(),
    }
}

#[tokio::test]
async fn forgetting_a_password_mails_a_reset_code_to_a_verified_address() {
    let world = verified_world().await;
    world.generator.queue("654321");

    world
        .start_password_reset()
        .execute(forgetting())
        .await
        .expect("a verified address may ask for a reset");

    let (to, purpose, code) = world
        .mailer
        .sent()
        .last()
        .cloned()
        .expect("a reset code must have been mailed");
    assert_eq!(to, NORMALISED);
    assert_eq!(purpose, OtpPurpose::ResetPassword);
    assert_eq!(code, "654321");
}

#[tokio::test]
async fn forgetting_a_password_for_an_unknown_address_mails_nothing_and_says_nothing() {
    let world = World::default();

    world
        .start_password_reset()
        .execute(StartPasswordResetInput {
            email: "nobody@example.test".to_owned(),
        })
        .await
        .expect("an unknown address is answered the same way as a known one");

    assert!(world.mailer.sent().is_empty());
}

#[tokio::test]
async fn forgetting_a_password_before_verifying_the_address_mails_nothing() {
    let world = World::default();
    world.generator.queue("123456");
    world
        .register()
        .execute(registration())
        .await
        .expect("registration succeeds");

    world
        .start_password_reset()
        .execute(forgetting())
        .await
        .expect("an unverified address is answered the same way");

    assert_eq!(world.mailer.sent().len(), 1);
    assert_eq!(world.mailer.sent()[0].1, OtpPurpose::VerifyEmail);
}

#[tokio::test]
async fn a_reset_replaces_the_password_and_kills_every_live_credential() {
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

    world.generator.queue("111222");
    world
        .start_password_reset()
        .execute(forgetting())
        .await
        .expect("a reset code is issued");
    world
        .reset_password()
        .execute(ResetPasswordInput {
            email: EMAIL.to_owned(),
            code: "111222".to_owned(),
            password: NEW_PASSWORD.to_owned(),
        })
        .await
        .expect("the reset completes");

    assert!(
        world
            .credentials
            .snapshot()
            .iter()
            .all(|row| row.revoked_at.is_some()),
        "a reset must leave no live credential behind"
    );

    let replayed = refused(
        world
            .refresh()
            .execute(RefreshInput {
                refresh_secret: session.refresh_secret,
                device_id: DEVICE.to_owned(),
            })
            .await,
        "a credential minted before the reset must be dead",
    );
    assert!(matches!(replayed, IamError::InvalidCredentials));

    world.generator.queue("333444");
    world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: NEW_PASSWORD.to_owned(),
        })
        .await
        .expect("the new password is the one that works now");
}

#[tokio::test]
async fn the_old_password_stops_working_after_a_reset() {
    let world = verified_world().await;
    world.generator.queue("111222");
    world
        .start_password_reset()
        .execute(forgetting())
        .await
        .expect("a reset code is issued");
    world
        .reset_password()
        .execute(ResetPasswordInput {
            email: EMAIL.to_owned(),
            code: "111222".to_owned(),
            password: NEW_PASSWORD.to_owned(),
        })
        .await
        .expect("the reset completes");

    let rejected = world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect_err("the password that was replaced must be refused");

    assert!(matches!(rejected, IamError::InvalidCredentials));
}

#[tokio::test]
async fn a_reset_code_from_another_purpose_is_refused() {
    let world = verified_world().await;
    world.generator.queue("654321");
    world
        .start_sign_in()
        .execute(StartSignInInput {
            email: EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
        })
        .await
        .expect("a sign-in code is issued");

    let rejected = world
        .reset_password()
        .execute(ResetPasswordInput {
            email: EMAIL.to_owned(),
            code: "654321".to_owned(),
            password: NEW_PASSWORD.to_owned(),
        })
        .await
        .expect_err("a sign-in code must not reset a password");

    assert!(matches!(rejected, IamError::ChallengeUnusable));
}

const OTHER_EMAIL: &str = "elsewhere@example.test";

#[tokio::test]
async fn changing_an_address_mails_the_new_one_and_leaves_the_old_in_place_until_confirmed() {
    let world = verified_world().await;
    let account = world.accounts.snapshot()[0].clone();
    world.generator.queue("222333");

    world
        .start_email_change()
        .execute(StartEmailChangeInput {
            account_id: account.id,
            email: OTHER_EMAIL.to_owned(),
        })
        .await
        .expect("a fresh address may be claimed");

    let (to, purpose, _) = world
        .mailer
        .sent()
        .last()
        .cloned()
        .expect("a code must have been mailed");
    assert_eq!(to, OTHER_EMAIL);
    assert_eq!(purpose, OtpPurpose::ChangeEmail);
    assert_eq!(world.accounts.snapshot()[0].email, NORMALISED);
}

#[tokio::test]
async fn confirming_an_address_change_moves_the_account_to_it() {
    let world = verified_world().await;
    let account = world.accounts.snapshot()[0].clone();
    world.generator.queue("222333");
    world
        .start_email_change()
        .execute(StartEmailChangeInput {
            account_id: account.id,
            email: OTHER_EMAIL.to_owned(),
        })
        .await
        .expect("a fresh address may be claimed");

    let adopted = world
        .confirm_email_change()
        .execute(ConfirmEmailChangeInput {
            account_id: account.id,
            code: "222333".to_owned(),
        })
        .await
        .expect("the code confirms the address");

    assert_eq!(adopted, OTHER_EMAIL);
    let stored = world.accounts.snapshot()[0].clone();
    assert_eq!(stored.email, OTHER_EMAIL);
    assert!(stored.pending_email.is_none());
}

#[tokio::test]
async fn an_address_another_account_already_holds_is_refused() {
    let world = verified_world().await;
    let account = world.accounts.snapshot()[0].clone();
    world.generator.queue("444555");
    world
        .register()
        .execute(RegisterInput {
            email: OTHER_EMAIL.to_owned(),
            password: PASSWORD.to_owned(),
            display_name: None,
        })
        .await
        .expect("a second account registers");

    let refused = world
        .start_email_change()
        .execute(StartEmailChangeInput {
            account_id: account.id,
            email: OTHER_EMAIL.to_owned(),
        })
        .await
        .expect_err("an address in use must be refused");

    assert!(matches!(refused, IamError::EmailAlreadyRegistered));
}

#[tokio::test]
async fn a_confirmation_without_a_claim_is_refused() {
    let world = verified_world().await;
    let account = world.accounts.snapshot()[0].clone();

    let refused = world
        .confirm_email_change()
        .execute(ConfirmEmailChangeInput {
            account_id: account.id,
            code: "222333".to_owned(),
        })
        .await
        .expect_err("there is no address waiting");

    assert!(matches!(refused, IamError::ChallengeUnusable));
}
