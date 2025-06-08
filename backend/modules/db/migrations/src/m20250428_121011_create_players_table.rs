use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
   async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Games::Table)
                    .if_not_exists()
                    .col(uuid(Games::Id).primary_key())
                    .col(uuid(Games::WhitePlayerId).not_null())
                    .col(uuid(Games::BlackPlayerId).not_null())
                    .col(string(Games::Result).not_null())
                    .col(timestamp_with_time_zone(Games::CreatedAt).not_null())
                  .col(timestamp_with_time_zone(Games::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GameMoves::Table)
                    .if_not_exists()
                    .col(uuid(GameMoves::Id).primary_key())
                    .col(uuid(GameMoves::GameId).not_null())
                    .col(integer(GameMoves::MoveNumber).not_null())
                    .col(string(GameMoves::Notation).not_null())
                    .col(json(GameMoves::BoardState))
                    .col(timestamp_with_time_zone(GameMoves::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_moves-game_id")
                            .from(GameMoves::Table, GameMoves::GameId)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
   }
   async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GameMoves::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(Games::Table).to_owned())
            .await
    }

#[derive(DeriveIden)]
enum Games {
    Table,
    Id,
   WhitePlayerId,
  BlackPlayerId,
    Result,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]enum GameMoves {
    Table,
    Id,
    GameId,
    MoveNumber,
    Notation,
    BoardState,
    CreatedAt,
}
