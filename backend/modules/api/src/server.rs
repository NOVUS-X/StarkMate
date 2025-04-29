// src/main.rs

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenv::dotenv;
use error::error::custom_json_error;
use std::env;

use crate::players::{add_player, delete_player, get_player_by_id, update_player};

/// Simple health-check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

/// Sample "hello" endpoint
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome to StarkMate API")
}

pub async fn main() -> std::io::Result<()> {
    // Load .env variables (e.g., DATABASE_URL, SERVER_ADDR)
    dotenv().ok();

    // Read server address from env or default
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Initialize logger (env_logger controlled via RUST_LOG)
    env_logger::init();

    println!("Starting StarkMate server at http://{}", &server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(custom_json_error))
            // Register health-check
            .route("/health", web::get().to(health))
            // Register greeting
            .route("/", web::get().to(greet))
            .service(web::scope("/v1/players").service(add_player).service(get_player_by_id).service(update_player).service(delete_player))
    })
    .bind(&server_addr)?
    .run()
    .await
}
