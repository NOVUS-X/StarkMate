// src/main.rs

mod routes;
mod models;
mod services;
mod utils;
mod db;

use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/starkmate")
        .await
        .expect("DB connection failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(routes::games::create_game)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
