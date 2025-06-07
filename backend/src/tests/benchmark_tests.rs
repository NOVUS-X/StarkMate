use chess_websocket_gateway::game::{init_game_state, join_room, send_move};
use std::time::Instant;
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        init_game_state();
    });
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn benchmark_room_creation() {
        setup();
        
        let start = Instant::now();
        
        for i in 0..1000 {
               let room_id = format!("bench-room-{}", i);
                let player_id = format!("player-{}", i);
                join_room(&room_id, &player_id, Some(format!("Player {}", i)))
                    .expect("Room creation should succeed");
            }
        
        let duration = start.elapsed();
        println!("Created 1000 rooms in {:?}", duration);
        
        // Should be able to create 1000 rooms in less than 1000ms
assert!(duration.as_millis() < 1000, "Room creation took {}ms", duration.as_millis());
    }

    #[test]
    fn benchmark_move_processing() {
        setup();
        
        // Set up a room with 2 players
        let _ = join_room("bench-moves", "player-1", Some("Alice".to_string()));
        let _ = join_room("bench-moves", "player-2", Some("Bob".to_string()));
        
        let start = Instant::now();
        
       // Example valid chess moves
let valid_moves = ["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6"];

for i in 0..1000 {
    let player_id = if i % 2 == 0 { "player-1" } else { "player-2" };
    let move_notation = valid_moves[i % valid_moves.len()];
    send_move("bench-moves", player_id, move_notation)
        .expect("Move should be processed successfully");
}
        
        let duration = start.elapsed();
        println!("Processed 1000 moves in {:?}", duration);
        
        // Should be able to process 1000 moves in less than 1 second
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn benchmark_concurrent_operations() {
        setup();
        
        let start = Instant::now();
        
        let handles: Vec<_> = (0..100)
            .map(|i| {
                std::thread::spawn(move || {
                    let room_id = format!("concurrent-bench-{}", i);
                    let _ = join_room(&room_id, "player-1", Some("Alice".to_string()));
                    let _ = join_room(&room_id, "player-2", Some("Bob".to_string()));
                    
                    for j in 0..10 {
                        let player_id = if j % 2 == 0 { "player-1" } else { "player-2" };
                        let move_notation = format!("move-{}", j);
                        let _ = send_move(&room_id, player_id, &move_notation);
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        println!("Completed 100 concurrent room operations in {:?}", duration);
        
        // Should complete concurrent operations in reasonable time
        assert!(duration.as_secs() < 5);
    }
}
