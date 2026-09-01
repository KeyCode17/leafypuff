use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// Sticker coordinates were stored in dp relative to the note. The note is not one width -- a
// tablet, a landscape phone and the Large text size all change it -- so a dp coordinate drifts the
// moment the note reflows. They are fractions of the layer from here on.
//
// Converting the rows that already exist needs a divisor, and the only honest one is the box the
// design drew them in: the 375dp frame less its 24dp gutters is a 327dp note, and the note the
// prototype staggers its drops into is about 200dp tall. Rows are converted only when a coordinate
// is outside -1..1, which no fraction can be but which every dp value beyond a hair of the edge
// is. That makes the migration idempotent and leaves an already-normalised row alone.
const REFERENCE_WIDTH: f32 = 327.0;
const REFERENCE_HEIGHT: f32 = 200.0;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();
        connection
            .execute_unprepared(&format!(
                "UPDATE stickers SET x = x / {REFERENCE_WIDTH} WHERE x > 1.0 OR x < -1.0"
            ))
            .await?;
        connection
            .execute_unprepared(&format!(
                "UPDATE stickers SET y = y / {REFERENCE_HEIGHT} WHERE y > 1.0 OR y < -1.0"
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();
        connection
            .execute_unprepared(&format!(
                "UPDATE stickers SET x = x * {REFERENCE_WIDTH} WHERE x <= 1.0 AND x >= -1.0"
            ))
            .await?;
        connection
            .execute_unprepared(&format!(
                "UPDATE stickers SET y = y * {REFERENCE_HEIGHT} WHERE y <= 1.0 AND y >= -1.0"
            ))
            .await?;
        Ok(())
    }
}
