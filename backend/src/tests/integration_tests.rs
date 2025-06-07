use chess_websocket_gateway::game::{get_game_log, join_room, leave_room, send_move, init_game_state};
use chess_websocket_gateway::models::{
    ClientMessage, ServerMessage, JoinRoomPayload, SendMovePayload, 
    LeaveRoomPayload, RequestGameLogPayload, GameStatus, PieceColor
};
use serde_json::{from_str, to_string};
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        init_game_state();
    });
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn test_join_room_success() {
        setup();
        
        let result = join_room("test-room-1", "player-1", Some("Alice".to_string()));
        assert!(result.is_ok());
        
        if let Ok(ServerMessage::RoomJoined { room_id, player_id, players, game_state }) = result {
            assert_eq!(room_id, "test-room-1");
            assert_eq!(player_id, "player-1");
            assert_eq!(players.len(), 1);
            assert_eq!(players[0].name, "Alice");
            assert_eq!(players[0].color, Some(PieceColor::White));
            assert!(game_state.is_none()); // Game doesn't start until 2 players
        } else {
            panic!("Expected RoomJoined message");
        }
    }

    #[test]
    fn test_join_room_second_player() {
        setup();
        
        // First player joins
        let _ = join_room("test-room-2", "player-1", Some("Alice".to_string()));
        
        // Second player joins
        let result = join_room("test-room-2", "player-2", Some("Bob".to_string()));
        assert!(result.is_ok());
        
        if let Ok(ServerMessage::RoomJoined { players, game_state, .. }) = result {
            assert_eq!(players.len(), 2);
            assert_eq!(players[1].color, Some(PieceColor::Black));
            assert!(game_state.is_some()); // Game starts with 2 players
            
            if let Some(state) = game_state {
                assert_eq!(state.current_turn, PieceColor::White);
                assert!(matches!(state.status, GameStatus::InProgress));
            }
        } else {
            panic!("Expected RoomJoined message");
        }
    }

    #[test]
    fn test_join_room_full() {
        setup();
        
        // Fill the room with 2 players
        let _ = join_room("test-room-3", "player-1", Some("Alice".to_string()));
        let _ = join_room("test-room-3", "player-2", Some("Bob".to_string()));
        
        // Try to add a third player
        let result = join_room("test-room-3", "player-3", Some("Charlie".to_string()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Room is full");
    }

    #[test]
    fn test_join_room_duplicate_player() {
        setup();
        
        // Player joins room
        let _ = join_room("test-room-4", "player-1", Some("Alice".to_string()));
        
        // Same player tries to join again
        let result = join_room("test-room-4", "player-1", Some("Alice".to_string()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Player is already in the room");
    }

    #[test]
    fn test_send_move_success() {
        setup();
        
        // Set up a room with 2 players
        let _ = join_room("test-room-5", "player-1", Some("Alice".to_string()));
        let _ = join_room("test-room-5", "player-2", Some("Bob".to_string()));
        
        // Send a move
        let result = send_move("test-room-5", "player-1", "e2e4");
        assert!(result.is_ok());
        
        if let Ok(ServerMessage::MoveMade { room_id, player_id, move_notation, game_state }) = result {
            assert_eq!(room_id, "test-room-5");
            assert_eq!(player_id, "player-1");
            assert_eq!(move_notation, "e2e4");
            assert_eq!(game_state.current_turn, PieceColor::Black); // Turn should switch
        } else {
            panic!("Expected MoveMade message");
        }
    }

    #[test]
    fn test_send_move_room_not_found() {
        setup();
        
        let result = send_move("nonexistent-room", "player-1", "e2e4");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Room not found");
    }

    #[test]
    fn test_send_move_player_not_in_room() {
        setup();
        
        // Create room with one player
        let _ = join_room("test-room-6", "player-1", Some("Alice".to_string()));
        
        // Try to make move with different player
        let result = send_move("test-room-6", "player-2", "e2e4");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Player not in room");
    }

    #[test]
    fn test_send_move_game_not_started() {
        setup();
        
        // Create room with only one player
        let _ = join_room("test-room-7", "player-1", Some("Alice".to_string()));
        
        // Try to make move before second player joins
        let result = send_move("test-room-7", "player-1", "e2e4");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Game not started");
    }

    #[test]
    fn test_leave_room_success() {
        setup();
        
        // Set up room with player
        let _ = join_room("test-room-8", "player-1", Some("Alice".to_string()));
        
        // Player leaves room
        let result = leave_room("test-room-8", "player-1");
        assert!(result.is_ok());
        
        if let Ok(ServerMessage::PlayerLeft { room_id, player_id }) = result {
            assert_eq!(room_id, "test-room-8");
            assert_eq!(player_id, "player-1");
        } else {
            panic!("Expected PlayerLeft message");
        }
    }

    #[test]
    fn test_leave_room_not_found() {
        setup();
        
        let result = leave_room("nonexistent-room", "player-1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Room not found");
    }

    #[test]
    fn test_leave_room_player_not_in_room() {
        setup();
        
        // Create room with one player
        let _ = join_room("test-room-9", "player-1", Some("Alice".to_string()));
        
        // Try to remove different player
        let result = leave_room("test-room-9", "player-2");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Player not in room");
    }

    #[test]
    fn test_get_game_log_success() {
        setup();
        
        // Set up room with 2 players and make some moves
        let _ = join_room("test-room-10", "player-1", Some("Alice".to_string()));
        let _ = join_room("test-room-10", "player-2", Some("Bob".to_string()));
        let _ = send_move("test-room-10", "player-1", "e2e4");
        let _ = send_move("test-room-10", "player-2", "e7e5");
        
        // Get game log
        let result = get_game_log("test-room-10");
        assert!(result.is_ok());
        
        if let Ok(ServerMessage::GameLog { room_id, moves }) = result {
            assert_eq!(room_id, "test-room-10");
            assert_eq!(moves.len(), 2);
            assert_eq!(moves[0].player_id, "player-1");
            assert_eq!(moves[0].move_notation, "e2e4");
            assert_eq!(moves[1].player_id, "player-2");
            assert_eq!(moves[1].move_notation, "e7e5");
        } else {
            panic!("Expected GameLog message");
        }
    }

    #[test]
    fn test_get_game_log_room_not_found() {
        setup();
        
        let result = get_game_log("nonexistent-room");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Room not found");
    }
}

#[cfg(test)]
mod model_tests {
    use super::*;

    #[test]
    fn test_client_message_serialization() {
        let join_message = ClientMessage::JoinRoom(JoinRoomPayload {
            room_id: "test-room".to_string(),
            player_id: "player-1".to_string(),
            player_name: Some("Alice".to_string()),
        });
        
        let json = to_string(&join_message).unwrap();
        let deserialized: ClientMessage = from_str(&json).unwrap();
        
        match deserialized {
            ClientMessage::JoinRoom(payload) => {
                assert_eq!(payload.room_id, "test-room");
                assert_eq!(payload.player_id, "player-1");
                assert_eq!(payload.player_name, Some("Alice".to_string()));
            }
            _ => panic!("Expected JoinRoom message"),
        }
    }

    #[test]
    fn test_send_move_message_serialization() {
        let move_message = ClientMessage::SendMove(SendMovePayload {
            room_id: "test-room".to_string(),
            player_id: "player-1".to_string(),
            move_notation: "e2e4".to_string(),
        });
        
        let json = to_string(&move_message).unwrap();
        let deserialized: ClientMessage = from_str(&json).unwrap();
        
        match deserialized {
            ClientMessage::SendMove(payload) => {
                assert_eq!(payload.room_id, "test-room");
                assert_eq!(payload.player_id, "player-1");
                assert_eq!(payload.move_notation, "e2e4");
            }
            _ => panic!("Expected SendMove message"),
        }
    }

    #[test]
    fn test_leave_room_message_serialization() {
        let leave_message = ClientMessage::LeaveRoom(LeaveRoomPayload {
            room_id: "test-room".to_string(),
            player_id: "player-1".to_string(),
        });
        
        let json = to_string(&leave_message).unwrap();
        let deserialized: ClientMessage = from_str(&json).unwrap();
        
        match deserialized {
            ClientMessage::LeaveRoom(payload) => {
                assert_eq!(payload.room_id, "test-room");
                assert_eq!(payload.player_id, "player-1");
            }
            _ => panic!("Expected LeaveRoom message"),
        }
    }

    #[test]
    fn test_request_game_log_message_serialization() {
        let log_message = ClientMessage::RequestGameLog(RequestGameLogPayload {
            room_id: "test-room".to_string(),
        });
        
        let json = to_string(&log_message).unwrap();
        let deserialized: ClientMessage = from_str(&json).unwrap();
        
        match deserialized {
            ClientMessage::RequestGameLog(payload) => {
                assert_eq!(payload.room_id, "test-room");
            }
            _ => panic!("Expected RequestGameLog message"),
        }
    }

    #[test]
    fn test_server_message_serialization() {
        let server_message = ServerMessage::Error {
            code: "TEST_ERROR".to_string(),
            message: "This is a test error".to_string(),
        };
        
        let json = to_string(&server_message).unwrap();
        let deserialized: ServerMessage = from_str(&json).unwrap();
        
        match deserialized {
            ServerMessage::Error { code, message } => {
                assert_eq!(code, "TEST_ERROR");
                assert_eq!(message, "This is a test error");
            }
            _ => panic!("Expected Error message"),
        }
    }

    #[test]
    fn test_invalid_message_format() {
        let invalid_json = r#"{"type": "InvalidType", "payload": {}}"#;
        let result: Result<ClientMessage, _> = from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_fields() {
        let incomplete_json = r#"{"type": "JoinRoom", "payload": {"room_id": "test"}}"#;
        let result: Result<ClientMessage, _> = from_str(incomplete_json);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod game_state_tests {
    use super::*;
    use chess_websocket_gateway::models::{GameState, PieceColor, GameStatus};

    #[test]
    fn test_new_game_initialization() {
        let game_state = GameState::new_game();
        
        assert_eq!(game_state.current_turn, PieceColor::White);
        assert!(matches!(game_state.status, GameStatus::InProgress));
        assert!(!game_state.board.is_empty());
        
        // Check that pawns are in correct positions
        assert!(game_state.board.contains_key("a2"));
        assert!(game_state.board.contains_key("h2"));
        assert!(game_state.board.contains_key("a7"));
        assert!(game_state.board.contains_key("h7"));
        
        // Check that major pieces are in correct positions
        assert!(game_state.board.contains_key("a1"));
        assert!(game_state.board.contains_key("h1"));
        assert!(game_state.board.contains_key("a8"));
        assert!(game_state.board.contains_key("h8"));
    }

    #[test]
    fn test_apply_move_turn_switching() {
        let mut game_state = GameState::new_game();
        
        assert_eq!(game_state.current_turn, PieceColor::White);
        
        let result = game_state.apply_move("e2e4");
        assert!(result.is_ok());
        assert_eq!(game_state.current_turn, PieceColor::Black);
        
        let result = game_state.apply_move("e7e5");
        assert!(result.is_ok());
        assert_eq!(game_state.current_turn, PieceColor::White);
    }
}

#[cfg(test)]
mod room_tests {
    use super::*;
    use chess_websocket_gateway::models::{Room, Player, PieceColor};

    #[test]
    fn test_room_creation() {
        let room = Room::new("test-room".to_string());
        
        assert_eq!(room.id, "test-room");
        assert!(room.players.is_empty());
        assert!(room.game_state.is_none());
        assert!(room.moves.is_empty());
    }

    #[test]
    fn test_add_first_player() {
        let mut room = Room::new("test-room".to_string());
        let player = Player {
            id: "player-1".to_string(),
            name: "Alice".to_string(),
            color: None,
        };
        
        let result = room.add_player(player);
        assert!(result.is_ok());
        assert_eq!(room.players.len(), 1);
        assert_eq!(room.players[0].color, Some(PieceColor::White));
        assert!(room.game_state.is_none()); // Game doesn't start until 2 players
    }

    #[test]
    fn test_add_second_player() {
        let mut room = Room::new("test-room".to_string());
        
        // Add first player
        let player1 = Player {
            id: "player-1".to_string(),
            name: "Alice".to_string(),
            color: None,
        };
        room.add_player(player1).unwrap();
        
        // Add second player
        let player2 = Player {
            id: "player-2".to_string(),
            name: "Bob".to_string(),
            color: None,
        };
        let result = room.add_player(player2);
        
        assert!(result.is_ok());
        assert_eq!(room.players.len(), 2);
        assert_eq!(room.players[1].color, Some(PieceColor::Black));
        assert!(room.game_state.is_some()); // Game starts with 2 players
    }

    #[test]
    fn test_add_third_player_fails() {
        let mut room = Room::new("test-room".to_string());
        
        // Add two players
        let player1 = Player { id: "player-1".to_string(), name: "Alice".to_string(), color: None };
        let player2 = Player { id: "player-2".to_string(), name: "Bob".to_string(), color: None };
        room.add_player(player1).unwrap();
        room.add_player(player2).unwrap();
        
        // Try to add third player
        let player3 = Player { id: "player-3".to_string(), name: "Charlie".to_string(), color: None };
        let result = room.add_player(player3);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Room is full");
    }

    #[test]
    fn test_add_duplicate_player_fails() {
        let mut room = Room::new("test-room".to_string());
        
        // Add player
        let player1 = Player { id: "player-1".to_string(), name: "Alice".to_string(), color: None };
        room.add_player(player1).unwrap();
        
        // Try to add same player again
        let player1_duplicate = Player { id: "player-1".to_string(), name: "Alice".to_string(), color: None };
        let result = room.add_player(player1_duplicate);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Player is already in the room");
    }

    #[test]
    fn test_remove_player() {
        let mut room = Room::new("test-room".to_string());
        
        // Add player
        let player = Player { id: "player-1".to_string(), name: "Alice".to_string(), color: None };
        room.add_player(player).unwrap();
        
        // Remove player
        let removed = room.remove_player("player-1");
        assert!(removed);
        assert!(room.players.is_empty());
        
        // Try to remove non-existent player
        let not_removed = room.remove_player("player-2");
        assert!(!not_removed);
    }

    #[test]
    fn test_add_move() {
        let mut room = Room::new("test-room".to_string());
        
        room.add_move("player-1".to_string(), "e2e4".to_string());
        
        assert_eq!(room.moves.len(), 1);
        assert_eq!(room.moves[0].player_id, "player-1");
        assert_eq!(room.moves[0].move_notation, "e2e4");
        assert!(room.moves[0].timestamp > 0);
    }
}

#[cfg(test)]
mod concurrent_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_room_joins() {
        setup();
        
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let room_id = format!("concurrent-room-{}", i);
                    let player_id = format!("player-{}", i);
                    join_room(&room_id, &player_id, Some(format!("Player {}", i)))
                })
            })
            .collect();
        
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_concurrent_moves_same_room() {
        setup();
        
        // Set up room with 2 players
        let _ = join_room("concurrent-moves", "player-1", Some("Alice".to_string()));
        let _ = join_room("concurrent-moves", "player-2", Some("Bob".to_string()));
        
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let player_id = if i % 2 == 0 { "player-1" } else { "player-2" };
                let move_notation = format!("move-{}", i);
                thread::spawn(move || {
                    send_move("concurrent-moves", player_id, &move_notation)
                })
            })
            .collect();
        
        let mut success_count = 0;
        for handle in handles {
            let result = handle.join().unwrap();
            if result.is_ok() {
                success_count += 1;
            }
        }
        
        // At least some moves should succeed
        assert!(success_count > 0);
    }
}
