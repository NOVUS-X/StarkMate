use sea_orm_migration::{prelude::*, schema::*};

// Need to import the Player enum from the other migration or a shared module
// For simplicity here, we redefine it. Ideally, it should be shared.
#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Placeholder for UP migration
        // Create the 'game' table
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_uuid(Game::Id))
                    .col(ColumnDef::new(Game::WhitePlayer).uuid().not_null())
                    .col(ColumnDef::new(Game::BlackPlayer).uuid().not_null())
                    .col(text(Game::Fen).not_null())
                    .col(ColumnDef::new(Game::Pgn).json_binary().not_null())
                    .col(string_len(Game::Result, 8).not_null())
                    .col(string_len(Game::Variant, 16).not_null())
                    .col(
                        ColumnDef::new(Game::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(integer(Game::DurationSec).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_white_player")
                            .from(Game::Table, Game::WhitePlayer)
                            .to(Player::Table, Player::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_black_player")
                            .from(Game::Table, Game::BlackPlayer)
                            .to(Player::Table, Player::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Add CHECK constraint using raw SQL
        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE "game" ADD CONSTRAINT "check_game_result" CHECK ("result" IN ('white', 'black', 'draw'))"#,
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_games_started_at")
                    .table(Game::Table)
                    .col(Game::StartedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_games_variant")
                    .table(Game::Table)
                    .col(Game::Variant)
                    .to_owned(),
            )
            .await?;

        // Create GIN index using raw SQL
        manager
            .get_connection()
            .execute_unprepared(r#"CREATE INDEX "idx_games_pgn_gin" ON "game" USING GIN ("pgn")"#)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Placeholder for DOWN migration
        // Drop indexes, constraints, foreign keys, and table in reverse order

        // Drop indexes (including GIN)
        manager
            .drop_index(Index::drop().name("idx_games_started_at").table(Game::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_games_variant").table(Game::Table).to_owned())
            .await?;
        manager
            .get_connection()
            .execute_unprepared(r#"DROP INDEX IF EXISTS "idx_games_pgn_gin""#)
            .await?;

        // Drop CHECK constraint (might need specific syntax depending on DB)
        // Assuming PostgreSQL:
        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE "game" DROP CONSTRAINT IF EXISTS "check_game_result""#)
            .await?;

        // Drop Foreign Keys (use the names defined in `up`)
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_game_white_player").table(Game::Table).to_owned())
            .await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_game_black_player").table(Game::Table).to_owned())
            .await?;

        // Drop the table
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    WhitePlayer,
    BlackPlayer,
    Fen,
    Pgn,
    Result,
    Variant,
    StartedAt,
    DurationSec,
} 