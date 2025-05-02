use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod routes;
use actix_cors::Cors;
use actix_web::http::header;
use env_logger;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("DB connection failed");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            // Add routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

use actix_web;
use api::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::main().await

}
