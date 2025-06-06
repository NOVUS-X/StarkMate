use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::models::{GameState, MoveRecord, Player, Room, ServerMessage};

// Type alias for the broadcast sender
type MessageSender = broadcast::Sender<ServerMessage>;

// Global game state
lazy_static::lazy_static! {
    static ref GAME_STATE: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState {
        rooms: HashMap::new(),
        message_senders: HashMap::new(),
    }));
}

// Game state structure
pub struct GameState {
    rooms: HashMap<String, Room>,
    message_senders: HashMap<String, MessageSender>,
}

// Initialize the game state
pub fn init_game_state() {
    // This function is called at startup to ensure the lazy_static is initialized
    let _guard = GAME_STATE.lock().unwrap();
    log::info!("Game state initialized");
}

// Get a clone of the message sender for a room
pub fn get_room_sender(room_id: &str) -> Option<MessageSender> {
    let state = GAME_STATE.lock().unwrap();
    state.message_senders.get(room_id).cloned()
}

// Create a new room
pub fn create_room() -> String {
    let room_id = Uuid::new_v4().to_string();
    let (tx, _) = broadcast::channel(100); // Buffer size of 100 messages
    
    let mut state = GAME_STATE.lock().unwrap();
    state.rooms.insert(room_id.clone(), Room::new(room_id.clone()));
    state.message_senders.insert(room_id.clone(), tx);
    
    room_id
}

// Join a room
pub fn join_room(room_id: &str, player_id: &str, player_name: Option<String>) -> Result<ServerMessage, String> {
    let mut state = GAME_STATE.lock().unwrap();
    
    // Check if room exists, create if not
    if !state.rooms.contains_key(room_id) {
        let new_room_id = room_id.to_string();
        let (tx, _) = broadcast::channel(100);
        state.rooms.insert(new_room_id.clone(), Room::new(new_room_id.clone()));
        state.message_senders.insert(new_room_id, tx);
    }
    
    let room = state.rooms.get_mut(room_id).unwrap();
    
    // Create player
    let player = Player {
        id: player_id.to_string(),
        name: player_name.unwrap_or_else(|| format!("Player {}", player_id)),
        color: None,
    };
    
    // Add player to room
    room.add_player(player)?;
    
    // Create response message
    let response = ServerMessage::RoomJoined {
        room_id: room_id.to_string(),
        player_id: player_id.to_string(),
        players: room.players.clone(),
        game_state: room.game_state.clone(),
    };
    
    // Broadcast to other players in the room
    if let Some(sender) = state.message_senders.get(room_id) {
        let _ = sender.send(response.clone());
    }
    
    Ok(response)
}

// Send a move
pub fn send_move(room_id: &str, player_id: &str, move_notation: &str) -> Result<ServerMessage, String> {
    let mut state = GAME_STATE.lock().unwrap();
    
    // Check if room exists
    let room = state.rooms.get_mut(room_id).ok_or_else(|| "Room not found".to_string())?;
    
    // Check if player is in the room
    if !room.players.iter().any(|p| p.id == player_id) {
        return Err("Player not in room".to_string());
    }
    
    // Check if game has started
    let game_state = room.game_state.as_mut().ok_or_else(|| "Game not started".to_string())?;
    
    // Apply the move
    game_state.apply_move(move_notation)?;
    
    // Record the move
    room.add_move(player_id.to_string(), move_notation.to_string());
    
    // Create response message
    let response = ServerMessage::MoveMade {
        room_id: room_id.to_string(),
        player_id: player_id.to_string(),
        move_notation: move_notation.to_string(),
        game_state: game_state.clone(),
    };
    
    // Broadcast to all players in the room
    if let Some(sender) = state.message_senders.get(room_id) {
        let _ = sender.send(response.clone());
    }
    
    Ok(response)
}

// Leave a room
pub fn leave_room(room_id: &str, player_id: &str) -> Result<ServerMessage, String> {
    let mut state = GAME_STATE.lock().unwrap();
    
    // Check if room exists
    let room = state.rooms.get_mut(room_id).ok_or_else(|| "Room not found".to_string())?;
    
    // Remove player from room
    if !room.remove_player(player_id) {
        return Err("Player not in room".to_string());
    }
    
    // Create response message
    let response = ServerMessage::PlayerLeft {
        room_id: room_id.to_string(),
        player_id: player_id.to_string(),
    };
    
    // Broadcast to all players in the room
    if let Some(sender) = state.message_senders.get(room_id) {
        let _ = sender.send(response.clone());
    }
    
    // Clean up empty rooms
    if room.players.is_empty() {
        state.rooms.remove(room_id);
        state.message_senders.remove(room_id);
    }
    
    Ok(response)
}

// Get game log
pub fn get_game_log(room_id: &str) -> Result<ServerMessage, String> {
    let state = GAME_STATE.lock().unwrap();
    
    // Check if room exists
    let room = state.rooms.get(room_id).ok_or_else(|| "Room not found".to_string())?;
    
    // Create response message
    let response = ServerMessage::GameLog {
        room_id: room_id.to_string(),
        moves: room.moves.clone(),
    };
    
    Ok(response)
}

// Database integration functions
// These are placeholders for future implementation

pub fn save_game_to_db(_room_id: &str) -> Result<(), String> {
    // In a real implementation, this would save the game state to a database
    Ok(())
}

pub fn load_game_from_db(_room_id: &str) -> Result<Room, String> {
    // In a real implementation, this would load the game state from a database
    Err("Not implemented".to_string())
}
