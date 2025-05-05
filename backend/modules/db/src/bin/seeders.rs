use db::entity::player::{self, Player};
use db::entity::sea_orm_active_enums::GameStatus;
use db::entity::{game, player};
use rand::Rng;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use db::DbResult;
use sea_orm::{ConnectionTrait, DbConn, QuerySelect};
use rand::prelude::*;
use std::env;
use std::time::Instant;

async fn seed_players(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    db.execute_unprepared("TRUNCATE TABLE smdb.game, smdb.player CASCADE;")
        .await?;
    println!("Existing data cleared.");

    const NUM_PLAYERS: usize = 100;
    println!("Seeding {} players...", NUM_PLAYERS);

    let players: Vec<player::ActiveModel> = (0..NUM_PLAYERS)
        .map(|i| {
            let player_id = Uuid::new_v4();
            player::ActiveModel {
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
                created_at: Default::default(),
                updated_at: Default::default(),
            }
        })
        .collect();

    Player::insert_many(players).exec(&db).await?;
    println!("Inserted {} players", NUM_PLAYERS);

    // Fetch player IDs after insertion for game seeding
    let player_ids: Vec<Uuid> = Player::find()
        .select_only()
        .column(player::Column::Id)
        .into_tuple()
        .all(&db)
        .await?;

    const NUM_GAMES: usize = 500;

    Ok(())
}

const NUM_PLAYERS: usize = 100; // Number of players to seed
const NUM_GAMES_PER_PLAYER_PAIR: usize = 5; // Number of games between each distinct player pair

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("Starting database seeding...");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url).await?;

    // --- Clear Existing Data ---
    println!("Clearing existing data from smdb schema...");
    db.execute_unprepared("TRUNCATE TABLE smdb.game, smdb.player CASCADE;").await?; // Corrected schema
    println!("Existing data cleared.");

    // --- Seed Players (Batch Insert) ---
    let mut rng = thread_rng();
    let start_players = Instant::now();
    println!("Seeding {} players...", NUM_PLAYERS);

    let player_models: Vec<player::ActiveModel> = (0..NUM_PLAYERS).map(|i| {
        player::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(format!("Player_{}", i + 1)),
            email: Set(format!("player{}@example.com", i + 1)),
            password_hash: Set(b"dummy_hash".to_vec()),
            biography: Set(format!("Biography for Player {}", i + 1)),
            country: Set("USA".to_string()),
            flair: Set("GM".to_string()),
            real_name: Set(format!("Real Name {}", i + 1)),
            location: Set("New York, NY".to_string()),
            fide_rating: Set(rng.gen_range(800..2800)),
            social_links: Set(vec![format!("http://twitter.com/player{}", i+1)]),
            ..Default::default() // Ensure other defaults are handled if any added later
        }
    }).collect();

    // Insert players in batches
    let insert_result = Player::insert_many(player_models).exec(&db).await?;
    println!("Inserted {} players in {:.2?}.", NUM_PLAYERS, start_players.elapsed());

    // Fetch the IDs of the newly created players
    let players = Player::find().limit(NUM_PLAYERS as u64).all(&db).await?;
    let player_ids: Vec<Uuid> = players.into_iter().map(|p| p.id).collect();

    if player_ids.len() < 2 {
        eprintln!("Warning: Need at least 2 players to seed games. Only {} players were seeded.", player_ids.len());
        return Ok(());
    }

    // --- Seed Games ---
    // ... rest of the file ... 