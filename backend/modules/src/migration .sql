-- backend/modules/src/migration.sql

-- Assuming this is the relevant game creation table schema
CREATE TYPE game_variant AS ENUM ('standard', 'chess960', 'three_check');

CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY,
    white_player_id UUID NOT NULL,
    black_player_id UUID,
    variant game_variant NOT NULL,
    fen TEXT NOT NULL,
    rated BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT now()
);



