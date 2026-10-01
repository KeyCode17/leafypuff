use api_testing::media_repositories::{InMemoryMedia, InMemoryObjects};
use leafypuff_api::application::media::SweepOrphanedMedia;
use leafypuff_api::domain::media::{MediaObject, MediaRepository, ObjectKey, ObjectStore, Variant};
use std::sync::Arc;
use uuid::Uuid;

async fn seed(
    objects: &InMemoryObjects,
    media: &InMemoryMedia,
    account: Uuid,
    entry: Uuid,
) -> Uuid {
    let photo = Uuid::new_v4();
    for variant in Variant::ALL {
        objects
            .put(&ObjectKey::new(account, photo, variant), vec![1, 2, 3])
            .await
            .expect("stored");
        media
            .record(MediaObject {
                photo_id: photo,
                account_id: account,
                entry_id: entry,
                variant,
                byte_len: 3,
                ciphertext_hash: "h".to_owned(),
                created_at_ms: 0,
            })
            .await
            .expect("recorded");
    }
    photo
}

#[tokio::test]
async fn the_sweep_forgets_media_of_tombstoned_entries_and_keeps_the_rest() {
    let objects = InMemoryObjects::default();
    let media = InMemoryMedia::default();
    let account = Uuid::new_v4();
    let live_entry = Uuid::new_v4();
    let dead_entry = Uuid::new_v4();

    let live_photo = seed(&objects, &media, account, live_entry).await;
    let dead_photo = seed(&objects, &media, account, dead_entry).await;
    media.tombstone(dead_entry);

    let swept = SweepOrphanedMedia::new(Arc::new(objects.clone()), Arc::new(media.clone()))
        .execute()
        .await
        .expect("sweep runs");

    assert_eq!(swept, 1);
    assert!(
        media
            .find(account, dead_photo)
            .await
            .expect("find")
            .is_empty()
    );
    assert_eq!(
        media.find(account, live_photo).await.expect("find").len(),
        2
    );
    assert!(
        objects
            .get(&ObjectKey::new(account, dead_photo, Variant::Original))
            .await
            .is_err()
    );
    assert!(
        objects
            .get(&ObjectKey::new(account, live_photo, Variant::Original))
            .await
            .is_ok()
    );
}
