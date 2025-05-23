use super::Chess960Position;

pub struct FenValidator;

impl FenValidator {
    /// Validate Chess960 FEN string
    pub fn validate_chess960_fen(fen: &str) -> Result<(), String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        
        if parts.len() != 6 {
            return Err("FEN must have 6 parts".to_string());
        }
        
        let board = parts[0];
        let ranks: Vec<&str> = board.split('/').collect();
        
        if ranks.len() != 8 {
            return Err("Board must have 8 ranks".to_string());
        }
        
        // Validate back rank (Chess960 constraints)
        Self::validate_back_rank(ranks[0])?;
        Self::validate_back_rank(ranks[7])?;
        
        Ok(())
    }
    
    /// Validate back rank follows Chess960 rules
    fn validate_back_rank(rank: &str) -> Result<(), String> {
        let pieces: Vec<char> = rank.chars().collect();
        
        if pieces.len() != 8 {
            return Err("Back rank must have 8 pieces".to_string());
        }
        
        // Check bishops on opposite colors
        let bishop_positions: Vec<usize> = pieces
            .iter()
            .enumerate()
            .filter(|(_, &piece)| piece.to_ascii_lowercase() == 'b')
            .map(|(i, _)| i)
            .collect();
        
        if bishop_positions.len() != 2 {
            return Err("Must have exactly 2 bishops".to_string());
        }
        
        if (bishop_positions[0] + bishop_positions[1]) % 2 == 0 {
            return Err("Bishops must be on opposite colored squares".to_string());
        }
        
        // Check king between rooks
        let king_pos = pieces
            .iter()
            .position(|&piece| piece.to_ascii_lowercase() == 'k')
            .ok_or("King not found")?;
        
        let rook_positions: Vec<usize> = pieces
            .iter()
            .enumerate()
            .filter(|(_, &piece)| piece.to_ascii_lowercase() == 'r')
            .map(|(i, _)| i)
            .collect();
        
        if rook_positions.len() != 2 {
            return Err("Must have exactly 2 rooks".to_string());
        }
        
        if king_pos <= rook_positions[0] || king_pos >= rook_positions[1] {
            return Err("King must be between rooks".to_string());
        }
        
        Ok(())
    }
    
    /// Validate Chess960 position
    pub fn validate_position(position: &Chess960Position) -> Result<(), String> {
        if position.position_number < 1 || position.position_number > 960 {
            return Err("Position number must be between 1 and 960".to_string());
        }
        
        Self::validate_chess960_fen(&position.fen)?;
        
        Ok(())
    }
}