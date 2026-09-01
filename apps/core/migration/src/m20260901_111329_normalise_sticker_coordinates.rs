use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

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
