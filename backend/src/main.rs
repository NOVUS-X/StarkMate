use actix_web::{web, App, HttpServer, middleware::Logger};
use backend::chess960;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    println!("Starting StarkMate Backend with Chess960 support...");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(chess960::configure_routes)
            // Add other existing routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}