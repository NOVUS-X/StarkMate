pub enum ValidationError {
    InvalidFormat,
    IllegalMove,
    KingInCheck,
    // Add more as needed
}

pub fn validate_move(fen: &str, mv: &str) -> Result<String, ValidationError> {
    let state = parse_fen(fen)?;
    let new_state = try_apply_move(&state, mv)?;
    if is_in_check(&new_state, state.to_move) {
        return Err(ValidationError::KingInCheck);
    }
    Ok(serialize_fen(&new_state))
}