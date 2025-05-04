CREATE TABLE time_controls (
    game_id UUID PRIMARY KEY,
    white_remaining_time BIGINT NOT NULL,
    black_remaining_time BIGINT NOT NULL,
    initial_time BIGINT NOT NULL,
    increment BIGINT NOT NULL,
    delay BIGINT NOT NULL
);
