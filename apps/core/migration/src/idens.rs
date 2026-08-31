use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Entries {
    Table,
    Id,
    Date,
    Mood,
    Title,
    TitleNonce,
    Body,
    BodyNonce,
    Revision,
    Weather,
    Location,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Photos {
    Table,
    Id,
    EntryId,
    Path,
    Ordinal,
    TakenAt,
}

#[derive(DeriveIden)]
pub enum Stickers {
    Table,
    Id,
    EntryId,
    Kind,
    X,
    Y,
    Size,
    Rotation,
}

#[derive(DeriveIden)]
pub enum Tags {
    Table,
    EntryId,
    Tag,
}
