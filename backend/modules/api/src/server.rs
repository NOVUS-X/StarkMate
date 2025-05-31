// src/server.rs

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_cors::Cors;
use dotenv::dotenv;
use error::error::custom_json_error;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::Redoc;
use std::env;
use crate::players::{add_player, delete_player, find_player_by_id, update_player};
use crate::games::{create_game, get_game, make_move, list_games, join_game, abandon_game};
use crate::auth::{login, register, refresh_token, logout};
use crate::ai::{get_ai_suggestion, analyze_position};

mod openapi;
use openapi::ApiDoc;

/// Simple health-check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

/// Sample "hello" endpoint
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome to StarkMate API")
}

pub async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    // Load .env variables (e.g., DATABASE_URL, SERVER_ADDR)
    dotenv().ok();

    // Read server address from env or default
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Initialize logger (env_logger controlled via RUST_LOG)
    env_logger::init();

    println!("Starting StarkMate server at http://{}", &server_addr);

    HttpServer::new(move || {
        App::new()
            // Add your app_data first
            .app_data(web::JsonConfig::default().error_handler(custom_json_error))
            // Register your routes
            .route("/health", web::get().to(health))
            .route("/", web::get().to(greet))
            // Player routes
            .service(
                web::scope("/v1/players")
                    .service(add_player)
                    .service(find_player_by_id)
                    .service(update_player)
                    .service(delete_player),
            )
            // Game routes
            .service(
                web::scope("/v1/games")
                    .service(create_game)
                    .service(get_game)
                    .service(list_games)
                    .service(join_game)
                    .route("/{id}/move", web::put().to(make_move))
                    .service(abandon_game),
            )
            // Auth routes
            .service(
                web::scope("/v1/auth")
                    .service(login)
                    .service(register)
                    .service(refresh_token)
                    .service(logout),
            )
            // AI routes
            .service(
                web::scope("/v1/ai")
                    .service(get_ai_suggestion)
                    .service(analyze_position),
            )
            // Swagger UI integration
            .service(
                SwaggerUi::new("/api/docs/{_:.*}")
                    .url("/api/docs/openapi.json", openapi.clone())
                    .config(utoipa_swagger_ui::Config::default().try_it_out_enabled(true))
            )
            // ReDoc integration (alternative documentation UI)
            .service(
                Redoc::new("/api/redoc")
                    .url("/api/docs/openapi.json", openapi.clone())
            )
            // WebSocket documentation as static HTML
            .route("/api/docs/websocket", web::get().to(|| async {
                HttpResponse::Ok()
                    .content_type("text/markdown")
                    .body(openapi::websocket_documentation())
            }))
    })
    .bind(&server_addr)?
    .run()
    .await
}
