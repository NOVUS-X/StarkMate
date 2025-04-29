pub mod players{
    use actix_web::{get, post, put, web::{Json, Path}, HttpResponse};
    use dto::players::players::{NewPlayer, UpdatePlayer};
    use entity::player::{DisplayPlayer, UpdatedPlayer};
    use serde_json::json;
    
    use service::players::{add_player as add_new_player, get_player_by_id as get_single_player_by_id, update_player as update_player_by_id};
    use uuid::Uuid;

    #[post("")]
    pub async fn add_player(payload: Json<NewPlayer>) -> HttpResponse{
        let player = add_new_player(payload.0).await;

        match player {
            Ok(plyr) => HttpResponse::Ok().json(json!({
                "message":"New player added",
                "body":{
                    "player": DisplayPlayer::from(plyr)
                }
            })),
            Err(err) => err.error_response(),
        }
    }

    #[get("/{id}")]
    pub async fn get_player_by_id(id: Path<Uuid>) -> HttpResponse{
        let player = get_single_player_by_id(id.into_inner()).await;

        match player {
            Ok(plyr) => HttpResponse::Ok().json(json!({
                "message":"Player found",
                "body":{
                    "player": DisplayPlayer::from(plyr)
                }
            })),
            Err(err) => err.error_response(),
        }
    }

    #[put("/{id}")]
    pub async fn update_player(id: Path<Uuid>, payload: Json<UpdatePlayer>) -> HttpResponse{
        let player = update_player_by_id(id.into_inner(), payload.0).await;
        
        match player {
            Ok(plyr) => HttpResponse::Ok().json(json!({
                "message":"Player updated",
                "body":{
                    "player": UpdatedPlayer::from(plyr)
                }
            })),
            Err(err) => err.error_response(),
        }
    }
}