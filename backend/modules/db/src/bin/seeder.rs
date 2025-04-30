use db_entity::prelude::*;
use db_entity::{player, game};
use sea_orm::{*, prelude::*};
use std::env;
use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::Rng;
use chrono::{Utc, Duration};
use serde_json::json;

const NUM_PLAYERS: usize = 100;
const NUM_GAMES: usize = 5000;

// Basic starting FEN position
const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;

    println!("Clearing existing data...");
    // Use execute_unprepared for TRUNCATE as it's not directly supported by query builder
    // Make sure the schema is correct if not using the default 'public'
    // Using CASCADE to handle foreign keys if necessary
    db.execute_unprepared("TRUNCATE TABLE public.game, public.player CASCADE;").await?;
    println!("Existing data cleared.");

    println!("Seeding database...");

    // --- Seed Players ---
    let mut player_ids = Vec::with_capacity(NUM_PLAYERS);
    println!("Seeding {} players...", NUM_PLAYERS);
    for i in 0..NUM_PLAYERS {
        let player_id = Uuid::new_v4();
        let player = player::ActiveModel {
            id: Set(player_id),
            username: Set(format!("Player_{}", i + 1)),
            email: Set(format!("player{}@example.com", i + 1)),
            password_hash: Set(b"dummy_hash".to_vec()), // Dummy hash
            biography: Set(format!("Biography for Player {}", i + 1)),
            country: Set("USA".to_string()), // Dummy country
            flair: Set("GM".to_string()), // Dummy flair
            real_name: Set(format!("Real Name {}", i + 1)),
            location: Set("New York, NY".to_string()), // Dummy location
            fide_rating: Set(rand::thread_rng().gen_range(800..2800)), // Use fide_rating
            social_links: Set(vec!["http://twitter.com/player".to_string()]), // Dummy links
        };
        Player::insert(player).exec(&db).await?;
        player_ids.push(player_id);
        if (i + 1) % 20 == 0 {
            println!("  Inserted {}/{} players", i + 1, NUM_PLAYERS);
        }
    }
    println!("Players seeded successfully.");

    // --- Seed Games ---
    let mut rng = rand::thread_rng();
    let variants = vec!["Standard", "Chess960", "Atomic", "Crazyhouse"];
    let results = vec!["white", "black", "draw"];

    println!("Seeding {} games...", NUM_GAMES);
    for i in 0..NUM_GAMES {
        let white_player_id = *player_ids.choose(&mut rng).unwrap();
        let black_player_id = loop {
            let id = *player_ids.choose(&mut rng).unwrap();
            if id != white_player_id { // Ensure players are different
                break id;
            }
        };

        let started_at = Utc::now() - Duration::days(rng.gen_range(0..365));
        let duration_sec = rng.gen_range(30..3600); // 30 seconds to 1 hour

        let game = game::ActiveModel {
            id: Set(Uuid::new_v4()),
            white_player: Set(white_player_id),
            black_player: Set(black_player_id),
            fen: Set(STARTING_FEN.to_string()), // Simple FEN for now
            pgn: Set(json!({ "moves": "e4 c5 ..." })), // Dummy PGN
            result: Set(results.choose(&mut rng).unwrap().to_string()),
            variant: Set(variants.choose(&mut rng).unwrap().to_string()),
            started_at: Set(started_at.into()),
            duration_sec: Set(duration_sec),
        };

        Game::insert(game).exec(&db).await?;
        if (i + 1) % 500 == 0 {
            println!("  Inserted {}/{} games", i + 1, NUM_GAMES);
        }
    }
    println!("Games seeded successfully.");

    println!("Database seeding complete!");

    Ok(())
} 