use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chess960Position {
    pub position_number: u16,
    pub fen: String,
    pub piece_arrangement: String,
}

pub struct Chess960Generator;

impl Chess960Generator {
    /// Generate all 960 Chess960 starting positions
    pub fn generate_all_positions() -> Vec<Chess960Position> {
        let mut positions = Vec::with_capacity(960);
        
        for position_id in 1..=960 {
            let arrangement = Self::generate_position_from_id(position_id);
            let fen = Self::arrangement_to_fen(&arrangement);
            
            positions.push(Chess960Position {
                position_number: position_id,
                fen,
                piece_arrangement: arrangement,
            });
        }
        
        positions
    }
    
    /// Generate position arrangement from Chess960 position ID (1-960)
    fn generate_position_from_id(position_id: u16) -> String {
        assert!(position_id >= 1 && position_id <= 960, "Position ID must be between 1 and 960");
        
        let id = position_id - 1; // Convert to 0-based indexing
        
        // Decode the position using the Chess960 numbering system
        let mut pieces = ['_'; 8];
        
        // Place bishops on opposite colors
        let light_bishop_pos = (id % 4) as usize * 2 + 1; // Odd squares (1,3,5,7)
        let dark_bishop_pos = ((id / 4) % 4) as usize * 2; // Even squares (0,2,4,6)
        
        pieces[light_bishop_pos] = 'B';
        pieces[dark_bishop_pos] = 'B';
        
        // Place queen in remaining positions
        let remaining_positions: Vec<usize> = (0..8)
            .filter(|&i| pieces[i] == '_')
            .collect();
        
        let queen_index = ((id / 16) % 6) as usize;
        pieces[remaining_positions[queen_index]] = 'Q';
        
        // Place knights in remaining positions
        let remaining_positions: Vec<usize> = (0..8)
            .filter(|&i| pieces[i] == '_')
            .collect();
        
        let knight_combo = ((id / 96) % 10) as usize;
        let (knight1_idx, knight2_idx) = Self::get_knight_positions(knight_combo);
        
        pieces[remaining_positions[knight1_idx]] = 'N';
        pieces[remaining_positions[knight2_idx]] = 'N';
        
        // Place rooks and king (king must be between rooks)
        let remaining_positions: Vec<usize> = (0..8)
            .filter(|&i| pieces[i] == '_')
            .collect();
        
        // The three remaining positions are for R-K-R
        pieces[remaining_positions[0]] = 'R';
        pieces[remaining_positions[1]] = 'K';
        pieces[remaining_positions[2]] = 'R';
        
        pieces.iter().collect()
    }
    
    /// Get knight positions for given combination index (0-9)
    fn get_knight_positions(combo: usize) -> (usize, usize) {
        let combinations = [
            (0, 1), (0, 2), (0, 3), (0, 4),
            (1, 2), (1, 3), (1, 4),
            (2, 3), (2, 4),
            (3, 4)
        ];
        combinations[combo]
    }
    
    /// Convert piece arrangement to FEN notation
    fn arrangement_to_fen(arrangement: &str) -> String {
        let back_rank = arrangement.to_lowercase();
        let white_pieces = arrangement;
        
        format!(
            "{}/pppppppp/8/8/8/8/PPPPPPPP/{} w KQkq - 0 1",
            back_rank,
            white_pieces
        )
    }
    
    /// Get specific position by number
    pub fn get_position(position_number: u16) -> Option<Chess960Position> {
        if position_number < 1 || position_number > 960 {
            return None;
        }
        
        let arrangement = Self::generate_position_from_id(position_number);
        let fen = Self::arrangement_to_fen(&arrangement);
        
        Some(Chess960Position {
            position_number,
            fen,
            piece_arrangement: arrangement,
        })
    }
    
    /// Generate random Chess960 position
    pub fn get_random_position() -> Chess960Position {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let position_number = rng.gen_range(1..=960);
        Self::get_position(position_number).unwrap()
    }
}