#![cfg(feature = "sqlite")]

use leafypuff_core::infrastructure::db;
use leafypuff_core::infrastructure::entity::{entries, photos, stickers, tags};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[tokio::test]
async fn entities_round_trip_against_the_migrated_schema() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let path = dir
        .path()
        .join("diary.sqlite")
        .to_string_lossy()
        .into_owned();
    let connection = db::open(&path).await.expect("a temp file opens");
    db::run_migrations(&connection)
        .await
        .expect("migrations apply");

    entries::ActiveModel {
        id: Set("11111111-1111-4111-8111-111111111111".to_owned()),
        date: Set("2026-08-31".to_owned()),
        mood: Set("loved".to_owned()),
        title: Set(b"Quiet morning".to_vec()),
        title_nonce: Set(None),
        body: Set(b"Tea on the balcony.".to_vec()),
        body_nonce: Set(None),
        revision: Set(0),
        weather: Set(Some("sunny".to_owned())),
        location: Set(Some("home".to_owned())),
        created_at: Set("2026-08-31T08:00:00Z".to_owned()),
        updated_at: Set("2026-08-31T08:00:00Z".to_owned()),
        synced_at: Set(None),
        deleted_at: Set(None),
    }
    .insert(&connection)
    .await
    .expect("an entry row inserts");

    photos::ActiveModel {
        place_x: Set(None),
        place_y: Set(None),
        place_size: Set(None),
        place_rotation: Set(None),
        place_crop_x: Set(None),
        place_crop_y: Set(None),
        place_crop_width: Set(None),
        place_ratio: Set(None),
        crop_x: Set(None),
        crop_y: Set(None),
        crop_width: Set(None),
        id: Set("photo-1".to_owned()),
        entry_id: Set("11111111-1111-4111-8111-111111111111".to_owned()),
        path: Set("/photos/one.jpg".to_owned()),
        ordinal: Set(0),
        taken_at: Set(None),
    }
    .insert(&connection)
    .await
    .expect("a photo row inserts");

    stickers::ActiveModel {
        id: Set("sticker-1".to_owned()),
        entry_id: Set("11111111-1111-4111-8111-111111111111".to_owned()),
        kind: Set("bunSleep".to_owned()),
        x: Set(12.5),
        y: Set(48.25),
        size: Set(64.0),
        rotation: Set(90.0),
    }
    .insert(&connection)
    .await
    .expect("a sticker row inserts");

    tags::ActiveModel {
        entry_id: Set("11111111-1111-4111-8111-111111111111".to_owned()),
        tag: Set("#slowday".to_owned()),
    }
    .insert(&connection)
    .await
    .expect("a tag row inserts");

    let stored = entries::Entity::find_by_id("11111111-1111-4111-8111-111111111111")
        .one(&connection)
        .await
        .expect("the entry reads back")
        .expect("the entry exists");
    assert_eq!(stored.title, b"Quiet morning".to_vec());
    assert!(stored.title_nonce.is_none());
    assert_eq!(stored.revision, 0);

    let sticker = stickers::Entity::find_by_id("sticker-1")
        .one(&connection)
        .await
        .expect("the sticker reads back")
        .expect("the sticker exists");
    assert!((sticker.size - 64.0).abs() < f32::EPSILON);
    assert!((sticker.rotation - 90.0).abs() < f32::EPSILON);

    assert_eq!(
        photos::Entity::find()
            .all(&connection)
            .await
            .expect("photos read")
            .len(),
        1
    );
    assert_eq!(
        tags::Entity::find()
            .all(&connection)
            .await
            .expect("tags read")
            .len(),
        1
    );
}
