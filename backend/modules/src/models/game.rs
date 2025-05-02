

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameVariant {
    Standard,
    Chess960,
    ThreeCheck,
}

#[derive(Debug, Deserialize)]
pub struct TimeControl {
    pub initial: u32,
    pub increment: u32,
    pub delay_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub players: Vec<String>, // usernames or IDs
    pub variant: GameVariant,
    pub time_control: TimeControl,
    pub rated: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateGameResponse {
    pub game_id: String,
    pub session_token: String,
    pub initial_state: String,
    pub player_assignments: HashMap<String, String>,
    pub join_url: String,
}
