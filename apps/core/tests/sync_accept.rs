#![cfg(feature = "sync")]

use leafypuff_core::infrastructure::db;
use leafypuff_core::infrastructure::entity::{entries, stickers, tags};
use leafypuff_core::infrastructure::sync_outbox::{Carried, InboundSticker, SyncOutbox};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

const ENTRY_ID: &str = "3f2a91c0-0000-4000-8000-0000000000ee";
const STAMP: &str = "2026-09-01T08:00:00+00:00";

async fn outbox() -> (tempfile::TempDir, DatabaseConnection, SyncOutbox) {
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
    let outbox = SyncOutbox::new(connection.clone());
    (dir, connection, outbox)
}

fn inbound_entry() -> entries::ActiveModel {
    entries::ActiveModel {
        id: ActiveValue::Set(ENTRY_ID.to_owned()),
        date: ActiveValue::Set("2026-09-01".to_owned()),
        mood: ActiveValue::Set("calm".to_owned()),
        title: ActiveValue::Set(vec![1, 2, 3]),
        title_nonce: ActiveValue::Set(Some(vec![7; 24])),
        body: ActiveValue::Set(vec![4, 5, 6]),
        body_nonce: ActiveValue::Set(Some(vec![8; 24])),
        revision: ActiveValue::Set(0),
        weather: ActiveValue::Set(Some("sunny".to_owned())),
        location: ActiveValue::Set(Some("home".to_owned())),
        created_at: ActiveValue::Set(STAMP.to_owned()),
        updated_at: ActiveValue::Set(STAMP.to_owned()),
        synced_at: ActiveValue::Set(Some(STAMP.to_owned())),
    }
}

fn carried(labels: &[&str], stickers: Vec<InboundSticker>) -> Carried {
    Carried {
        entry_id: ENTRY_ID.to_owned(),
        tags: labels.iter().map(|tag| (*tag).to_owned()).collect(),
        stickers,
        photos: Vec::new(),
    }
}

fn sticker(key: &str) -> InboundSticker {
    InboundSticker {
        key: key.to_owned(),
        kind: "Heart".to_owned(),
        x: 0.25,
        y: 0.5,
        size: 64.0,
        rotation: 0.0,
    }
}

async fn stored_tags(connection: &DatabaseConnection) -> Vec<String> {
    let mut held: Vec<String> = tags::Entity::find()
        .filter(tags::Column::EntryId.eq(ENTRY_ID.to_owned()))
        .all(connection)
        .await
        .expect("the tags read")
        .into_iter()
        .map(|row| row.tag)
        .collect();
    held.sort();
    held
}

async fn stored_stickers(connection: &DatabaseConnection) -> Vec<String> {
    let mut held: Vec<String> = stickers::Entity::find()
        .filter(stickers::Column::EntryId.eq(ENTRY_ID.to_owned()))
        .all(connection)
        .await
        .expect("the stickers read")
        .into_iter()
        .map(|row| row.id)
        .collect();
    held.sort();
    held
}

#[tokio::test]
async fn an_accepted_entry_keeps_its_tags_stickers_weather_and_location() {
    let (_dir, connection, outbox) = outbox().await;

    outbox
        .accept(
            inbound_entry(),
            &carried(&["#rain", "#home"], vec![sticker("heart-0")]),
        )
        .await
        .expect("the entry is accepted");

    let row = entries::Entity::find_by_id(ENTRY_ID.to_owned())
        .one(&connection)
        .await
        .expect("the entry reads")
        .expect("the entry is there");

    assert_eq!(row.weather, Some("sunny".to_owned()));
    assert_eq!(row.location, Some("home".to_owned()));
    assert_eq!(stored_tags(&connection).await, vec!["#home", "#rain"]);
    assert_eq!(stored_stickers(&connection).await, vec!["heart-0"]);
}

#[tokio::test]
async fn a_second_arrival_replaces_the_tags_and_stickers_rather_than_adding_to_them() {
    let (_dir, connection, outbox) = outbox().await;

    outbox
        .accept(
            inbound_entry(),
            &carried(
                &["#rain", "#home"],
                vec![sticker("heart-0"), sticker("star-1")],
            ),
        )
        .await
        .expect("the first arrival");
    outbox
        .accept(
            inbound_entry(),
            &carried(&["#home"], vec![sticker("star-1")]),
        )
        .await
        .expect("the second arrival");

    // A tag the writing device no longer names is a tag its owner removed.
    assert_eq!(stored_tags(&connection).await, vec!["#home"]);
    assert_eq!(stored_stickers(&connection).await, vec!["star-1"]);
}
