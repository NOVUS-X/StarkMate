#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_all_positions() {
        let positions = Chess960Generator::generate_all_positions();
        assert_eq!(positions.len(), 960);
        
        // Check all position numbers are unique
        let mut numbers: Vec<u16> = positions.iter().map(|p| p.position_number).collect();
        numbers.sort();
        assert_eq!(numbers, (1..=960).collect::<Vec<u16>>());
    }
    
    #[test]
    fn test_specific_positions() {
        // Test position 1 (standard starting position equivalent)
        let pos1 = Chess960Generator::get_position(1).unwrap();
        assert!(pos1.fen.contains("rnbqkbnr"));
        
        // Test random positions
        for i in [1, 100, 500, 960] {
            let position = Chess960Generator::get_position(i).unwrap();
            assert_eq!(position.position_number, i);
            assert!(FenValidator::validate_chess960_fen(&position.fen).is_ok());
        }
    }
    
    #[test]
    fn test_fen_validation() {
        let valid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(FenValidator::validate_chess960_fen(valid_fen).is_ok());
        
        let invalid_fen = "invalid";
        assert!(FenValidator::validate_chess960_fen(invalid_fen).is_err());
    }
    
    #[test]
    fn test_bishop_opposite_colors() {
        let positions = Chess960Generator::generate_all_positions();
        
        for position in positions.iter().take(100) { // Test first 100 positions
            let fen_parts: Vec<&str> = position.fen.split_whitespace().collect();
            let back_rank = fen_parts[0].split('/').last().unwrap();
            
            let bishop_positions: Vec<usize> = back_rank
                .chars()
                .enumerate()
                .filter(|(_, piece)| piece.to_ascii_lowercase() == 'b')
                .map(|(i, _)| i)
                .collect();
            
            assert_eq!(bishop_positions.len(), 2);
            assert_ne!((bishop_positions[0] + bishop_positions[1]) % 2, 0);
        }
    }
    
    #[test]
    fn test_king_between_rooks() {
        let positions = Chess960Generator::generate_all_positions();
        
        for position in positions.iter().take(100) { // Test first 100 positions
            let fen_parts: Vec<&str> = position.fen.split_whitespace().collect();
            let back_rank = fen_parts[0].split('/').last().unwrap();
            
            let king_pos = back_rank
                .chars()
                .position(|piece| piece.to_ascii_lowercase() == 'k')
                .unwrap();
            
            let rook_positions: Vec<usize> = back_rank
                .chars()
                .enumerate()
                .filter(|(_, piece)| piece.to_ascii_lowercase() == 'r')
                .map(|(i, _)| i)
                .collect();
            
            assert_eq!(rook_positions.len(), 2);
            assert!(king_pos > rook_positions[0] && king_pos < rook_positions[1]);
        }
    }
}