use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_game_white_player")
                    .table(Game::Table)
                    .col(Game::WhitePlayer)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_game_black_player")
                    .table(Game::Table)
                    .col(Game::BlackPlayer)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_game_id")
                    .table(Game::Table)
                    .col(Game::Id)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_game_white_player").table(Game::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_game_black_player").table(Game::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_game_id").table(Game::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
#[iden = "smdb"] // Specify the schema here
enum Game {
    #[iden = "game"] // Specify the table name here
    Table,
    Id,
    WhitePlayer,
    BlackPlayer,
    StartedAt,
    // Add other columns if needed for future migrations involving this table
}
