# Chess960 FEN Library

This module provides a comprehensive implementation for generating, validating, and managing Chess960 (Fischer Random Chess) starting positions.

## Features

- Generate all 960 unique Chess960 starting positions
- Convert positions to FEN notation
- Validate Chess960 FEN strings
- API endpoints for position retrieval
- JSON export/import functionality

## API Endpoints

- `GET /api/chess960/positions` - Get all positions
- `GET /api/chess960/positions/{number}` - Get specific position (1-960)
- `GET /api/chess960/random` - Get random position
- `POST /api/chess960/validate` - Validate FEN string

## Chess960 Rules

1. Bishops must be placed on opposite-colored squares
2. King must be placed between the two rooks
3. All other pieces are placed randomly
4. Results in exactly 960 unique starting positions

## Usage

```rust
use chess960::{Chess960Generator, FenValidator};

// Get specific position
let position = Chess960Generator::get_position(518).unwrap();
println!("FEN: {}", position.fen);

// Validate FEN
let is_valid = FenValidator::validate_chess960_fen(&position.fen).is_ok();