use actix_web::{post, web, HttpResponse, Responder};
use crate::models::game::*;
use crate::services::game_service::create_game_service;
use sqlx::PgPool;

#[post("/games/new")]
pub async fn create_game(
    payload: web::Json<CreateGameRequest>,
    db: web::Data<PgPool>,
) -> impl Responder {
    match create_game_service(payload.into_inner(), db.get_ref()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}
