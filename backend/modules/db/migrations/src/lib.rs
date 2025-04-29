pub use sea_orm_migration::prelude::*;

mod m20250428_121011_create_players_table;
mod m20250429_193845_make_player_fields_nullable;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250428_121011_create_players_table::Migration),
            Box::new(m20250429_193845_make_player_fields_nullable::Migration),
        ]
    }
}
