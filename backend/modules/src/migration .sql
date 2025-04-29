-- ============================================
-- StarkMate: Create 'games' table for chess matches

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================
-- Drop table only if you're testing locally
-- WARNING: Remove this in production!
-- ============================================
-- DROP TABLE IF EXISTS games;

-- ============================================
-- Create 'games' table
-- ============================================
CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    white_player TEXT NOT NULL,
    black_player TEXT NOT NULL,
    variant TEXT NOT NULL CHECK (variant IN ('standard', 'chess960', 'three-check')),
    fen TEXT NOT NULL,
    rated BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- Indexing for better performance on listings
-- ============================================
CREATE INDEX IF NOT EXISTS idx_games_created_at ON games(created_at);



