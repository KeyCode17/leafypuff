use api_testing::repositories::InMemoryAccounts;
use leafypuff_api::domain::iam::{AccountRepository, Profile};
use uuid::Uuid;

#[tokio::test]
async fn an_unset_profile_reads_as_empty() {
    let accounts = InMemoryAccounts::default();

    let held = accounts
        .profile(Uuid::new_v4())
        .await
        .expect("the profile reads");

    assert_eq!(held, Profile::default());
}

#[tokio::test]
async fn a_saved_profile_reads_back() {
    let accounts = InMemoryAccounts::default();
    let id = Uuid::new_v4();
    let photo = Uuid::new_v4();
    let wanted = Profile {
        sealed_profile: Some("c2VhbGVk".to_owned()),
        avatar_photo_id: Some(photo),
        updated_at_ms: 1_700_000_000_000,
    };

    accounts
        .save_profile(id, wanted.clone())
        .await
        .expect("the profile saves");

    assert_eq!(
        accounts.profile(id).await.expect("the profile reads"),
        wanted
    );
}

#[tokio::test]
async fn an_older_write_leaves_the_stored_profile_alone() {
    let accounts = InMemoryAccounts::default();
    let id = Uuid::new_v4();
    let kept = Profile {
        sealed_profile: Some("bmV3".to_owned()),
        avatar_photo_id: None,
        updated_at_ms: 2_000,
    };
    accounts
        .save_profile(id, kept.clone())
        .await
        .expect("the newer profile saves");

    let answered = accounts
        .save_profile(
            id,
            Profile {
                sealed_profile: Some("b2xk".to_owned()),
                avatar_photo_id: None,
                updated_at_ms: 1_000,
            },
        )
        .await
        .expect("the older profile is answered");

    assert_eq!(answered, kept);
    assert_eq!(accounts.profile(id).await.expect("the profile reads"), kept);
}

#[tokio::test]
async fn a_write_at_the_same_stamp_still_lands() {
    let accounts = InMemoryAccounts::default();
    let id = Uuid::new_v4();
    accounts
        .save_profile(
            id,
            Profile {
                sealed_profile: Some("Zmlyc3Q".to_owned()),
                avatar_photo_id: None,
                updated_at_ms: 5_000,
            },
        )
        .await
        .expect("the first profile saves");

    let replaced = Profile {
        sealed_profile: Some("c2Vjb25k".to_owned()),
        avatar_photo_id: None,
        updated_at_ms: 5_000,
    };
    accounts
        .save_profile(id, replaced.clone())
        .await
        .expect("the retry saves");

    assert_eq!(
        accounts.profile(id).await.expect("the profile reads"),
        replaced
    );
}
