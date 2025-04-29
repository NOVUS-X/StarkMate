use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let alter_table_statement = Table::alter()
        .table(Alias::new("player"))
        .modify_column(ColumnDef::new(Alias::new("biography")).null())
        .modify_column(ColumnDef::new(Alias::new("country")).null())
        .modify_column(ColumnDef::new(Alias::new("flair")).null())
        .modify_column(ColumnDef::new(Alias::new("real_name")).null())
        .modify_column(ColumnDef::new(Alias::new("location")).null())
        .modify_column(ColumnDef::new(Alias::new("fide_rating")).null())
        .modify_column(ColumnDef::new(Alias::new("social_links")).null())
        .to_owned();


        manager.alter_table(alter_table_statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let alter_table_statement = Table::alter()
        .table(Alias::new("player"))
        .modify_column(ColumnDef::new(Alias::new("biography")).not_null())
        .modify_column(ColumnDef::new(Alias::new("country")).not_null())
        .modify_column(ColumnDef::new(Alias::new("flair")).not_null())
        .modify_column(ColumnDef::new(Alias::new("real_name")).not_null())
        .modify_column(ColumnDef::new(Alias::new("location")).not_null())
        .modify_column(ColumnDef::new(Alias::new("fide_rating")).not_null())
        .modify_column(ColumnDef::new(Alias::new("social_links")).not_null())
        .to_owned();


        manager.alter_table(alter_table_statement).await
    }
}
