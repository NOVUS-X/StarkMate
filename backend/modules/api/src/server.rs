// src/main.rs

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenv::dotenv;
use error::error::custom_json_error;
use utoipa_swagger_ui::SwaggerUi;
use std::env;
use crate::players::{add_player, delete_player, find_player_by_id, update_player};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::players::add_player,
        crate::players::find_player_by_id,
        crate::players::update_player,
        crate::players::delete_player
    ),
    tags(
        (name = "StarkAPI", description = "Stark API")
    )
)]
struct ApiDoc;

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
            .service(
                web::scope("/v1/players")
                    .service(add_player)
                    .service(find_player_by_id)
                    .service(update_player)
                    .service(delete_player),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(&server_addr)?
    .run()
    .await
}
