suse sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "game_player_states")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub game_id: Uuid,
    pub move_number: i32,
    pub player_id: Uuid,
    pub state_json: Json,  
    pub timestamp: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def().to(super::game::Entity)
    }
}

impl ActiveModelBehavior for ActiveModel {}
