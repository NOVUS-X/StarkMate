# Chess960 FEN Library

## Overview

Complete implementation of Chess960 (Fischer Random Chess) starting positions library with 960 unique FEN strings.

## Features

- Generate all 960 valid Chess960 starting positions
- RESTful API endpoints for position retrieval
- FEN string validation
- Position verification against Chess960 rules
- JSON export functionality
- Comprehensive test suite

## API Endpoints

### GET /api/chess960/position
Get a Chess960 position by number or random

**Parameters:**
- `number` (optional): Position number (1-960)
- `random` (optional): Get random position (true/false)

### GET /api/chess960/fen/{number}
Get FEN string for specific position

### POST /api/chess960/verify
Verify if FEN is valid Chess960 position

### GET /api/chess960/stats
Get library statistics

### GET /api/chess960/export
Export complete library as JSON

## Usage Examples

```bash
# Get specific position
curl "http://localhost:8080/api/chess960/position?number=1"

# Get random position
curl "http://localhost:8080/api/chess960/position?random=true"

# Get FEN string
curl "http://localhost:8080/api/chess960/fen/1"

# Verify FEN
curl -X POST "http://localhost:8080/api/chess960/verify" \
  -H "Content-Type: application/json" \
  -d '{"fen":"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"}'