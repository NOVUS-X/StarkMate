use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let alter_table_statement = Table::alter()
        .table(Alias::new("player"))
        .add_column(ColumnDef::new(Alias::new("is_enabled")).boolean().default(true))
        .to_owned();


        manager.alter_table(alter_table_statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let alter_table_statement = Table::alter()
        .table(Alias::new("player"))
        .drop_column(Alias::new("is_enabled"))
        .to_owned();


        manager.alter_table(alter_table_statement).await
    }
}
