use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "game_moves")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,               
    pub game_id: Uuid,           
    pub move_number: i32,        
    pub san: String,            
    pub fen: String,             
    pub timestamp: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
