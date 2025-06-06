mod game;
mod handlers;
mod models;
mod websocket;

use std::env;
use tokio::net::TcpListener;
use websocket::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Get the address from environment or use default
    let addr = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    log::info!("Starting WebSocket server on {}", addr);
    
    // Initialize the game state
    game::init_game_state();
    
    // Create the TCP listener
    let listener = TcpListener::bind(&addr).await?;
    log::info!("WebSocket server listening on: {}", addr);
    
       // Accept connections
loop {
       match listener.accept().await {
                Ok((stream, addr)) => {
                    log::info!("New connection from: {}", addr);
                    
                    // Spawn a new task for each connection
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, addr).await {
                            log::error!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                    // Continue accepting connections despite errors
                }
            }
        }
    
    Ok(())
}
