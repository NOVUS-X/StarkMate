use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(pk_uuid(Player::Id))
                    .col(string(Player::Username).not_null().unique_key())
                    .col(string(Player::Email).not_null().unique_key())
                    .col(binary(Player::PasswordHash).not_null())
                    .col(text(Player::Biography))
                    .col(string(Player::Country))
                    .col(string(Player::Flair))
                    .col(string(Player::RealName))
                    .col(string(Player::Location))
                    .col(integer(Player::FIDERating))
                    .col(array(Player::SocialLinks, ColumnType::String(StringLen::N(150))))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    Biography,
    Country,
    Flair,
    RealName,
    Location,
    FIDERating,
    SocialLinks
}
