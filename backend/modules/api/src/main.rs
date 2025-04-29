// src/main.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenv::dotenv;
use std::env;

/// Simple health-check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

/// Sample "hello" endpoint
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome to StarkMate API")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env variables (e.g., DATABASE_URL, SERVER_ADDR)
    dotenv().ok();

    // Read server address from env or default
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Initialize logger (env_logger controlled via RUST_LOG)
    env_logger::init();

    println!("Starting StarkMate server at http://{}", &server_addr);

    HttpServer::new(move || {
        App::new()
            // Register health-check
            .route("/health", web::get().to(health))
            // Register greeting
            .route("/", web::get().to(greet))
    })
    .bind(&server_addr)?
    .run()
    .await
}
