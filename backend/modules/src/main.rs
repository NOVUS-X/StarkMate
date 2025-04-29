use actix_web::{web, App, HttpServer};
use api::players::players::{add_player, get_player_by_id, update_player};
use error::error::custom_json_error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::JsonConfig::default().error_handler(custom_json_error))
        .service(web::scope("/v1/players").service(add_player).service(get_player_by_id).service(update_player))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
