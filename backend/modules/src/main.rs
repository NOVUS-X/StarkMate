use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

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
}
