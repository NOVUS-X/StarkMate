#[test]
fn test_parse_starting_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = parse_fen(fen).unwrap();
    // assert correct piece placements, to_move, etc.
}