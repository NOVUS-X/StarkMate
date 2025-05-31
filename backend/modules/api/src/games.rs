use actix_web::{
    HttpResponse, delete, get, post, put,
    web::{Json, Path, Query},
};
use dto::{
    games::{CreateGameRequest, GameDisplayDTO, MakeMoveRequest, JoinGameRequest, GameStatus},
    responses::{InvalidCredentialsResponse, NotFoundResponse},
};
use error::error::ApiError;
use serde_json::json;
use validator::Validate;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/v1/games",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Game created successfully", body = GameDisplayDTO),
        (status = 400, description = "Invalid request parameters", body = InvalidCredentialsResponse),
        (status = 401, description = "Unauthorized", body = InvalidCredentialsResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Games"
)]
#[post("")]
pub async fn create_game(payload: Json<CreateGameRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would create a game in the database
            // For now, we'll just return a mock response
            HttpResponse::Created().json(json!({
                "message": "Game created successfully",
                "data": {
                    "game": {
                        "id": Uuid::new_v4(),
                        "status": "waiting"
                    }
                }
            }))
        }
        Err(errors) => ApiError::ValidationError(errors).error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/games/{id}",
    params(
        ("id" = String, Path, description = "Game ID in UUID format", format = "uuid")
    ),
    responses(
        (status = 200, description = "Game found", body = GameDisplayDTO),
        (status = 404, description = "Game not found", body = NotFoundResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Games"
)]
#[get("/{id}")]
pub async fn get_game(id: Path<Uuid>) -> HttpResponse {
    // The real implementation would fetch the game from the database
    // For now, we'll just return a mock response
    HttpResponse::Ok().json(json!({
        "message": "Game found",
        "data": {
            "game": {
                "id": id.into_inner(),
                "status": "in_progress"
            }
        }
    }))
}

#[utoipa::path(
    put,
    path = "/v1/games/{id}/move",
    params(
        ("id" = String, Path, description = "Game ID in UUID format", format = "uuid")
    ),
    request_body = MakeMoveRequest,
    responses(
        (status = 200, description = "Move made successfully", body = GameDisplayDTO),
        (status = 400, description = "Invalid move", body = InvalidCredentialsResponse),
        (status = 404, description = "Game not found", body = NotFoundResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Games"
)]
#[put("/{id}/move")]
pub async fn make_move(id: Path<Uuid>, payload: Json<MakeMoveRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would validate and make the move
            // For now, we'll just return a mock response
            HttpResponse::Ok().json(json!({
                "message": "Move made successfully",
                "data": {
                    "game": {
                        "id": id.into_inner(),
                        "status": "in_progress",
                        "last_move": payload.0.chess_move
                    }
                }
            }))
        }
        Err(errors) => ApiError::ValidationError(errors).error_response(),
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListGamesQuery {
    #[schema(example = "waiting")]
    pub status: Option<String>,
    
    #[schema(value_type = Option<String>, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174000")]
    pub player_id: Option<Uuid>,
    
    #[schema(default = 1, example = 1)]
    pub page: Option<i32>,
    
    #[schema(default = 10, example = 10)]
    pub limit: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/v1/games",
    params(
        ("status" = Option<String>, Query, description = "Filter games by status (waiting, in_progress, completed, aborted)"),
        ("player_id" = Option<String>, Query, description = "Filter games by player ID", format = "uuid"),
        ("page" = Option<i32>, Query, description = "Page number for pagination"),
        ("limit" = Option<i32>, Query, description = "Number of items per page")
    ),
    responses(
        (status = 200, description = "List of games", body = Vec<GameDisplayDTO>)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Games"
)]
#[get("")]
pub async fn list_games(query: Query<ListGamesQuery>) -> HttpResponse {
    // The real implementation would fetch games from the database with filters
    // For now, we'll just return a mock response with filtering logic
    
    // Default pagination values
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    // Create a mock list of games
    let mut mock_games = vec![
        json!({
            "id": Uuid::new_v4(),
            "status": "waiting",
            "white_player_id": Uuid::new_v4(),
            "black_player_id": null,
            "created_at": "2025-05-31T10:00:00Z"
        }),
        json!({
            "id": Uuid::new_v4(),
            "status": "in_progress",
            "white_player_id": Uuid::new_v4(),
            "black_player_id": Uuid::new_v4(),
            "created_at": "2025-05-31T11:00:00Z"
        }),
        json!({
            "id": Uuid::new_v4(),
            "status": "completed",
            "white_player_id": Uuid::new_v4(),
            "black_player_id": Uuid::new_v4(),
            "created_at": "2025-05-31T09:00:00Z"
        })
    ];
    
    // Apply status filter if provided
    if let Some(status) = &query.status {
        mock_games.retain(|game| {
            game["status"].as_str().unwrap_or("") == status
        });
    }
    
    // Apply player_id filter if provided
    if let Some(player_id) = query.player_id {
        mock_games.retain(|game| {
            // Check if player is white or black player
            let white_id = game["white_player_id"].as_str().unwrap_or("");
            let black_id = game["black_player_id"].as_str().unwrap_or("");
            white_id == player_id.to_string() || black_id == player_id.to_string()
        });
    }
    
    // Apply pagination
    let total = mock_games.len();
    let start_idx = ((page - 1) * limit) as usize;
    let end_idx = (start_idx + limit as usize).min(total);
    
    // Get paginated subset (handle out of bounds)
    let paginated_games = if start_idx < total {
        mock_games[start_idx..end_idx].to_vec()
    } else {
        Vec::new()
    };
    
    HttpResponse::Ok().json(json!({
        "message": "Games found",
        "data": {
            "games": paginated_games,
            "pagination": {
                "total": total,
                "page": page,
                "limit": limit,
                "pages": (total as f32 / limit as f32).ceil() as i32
            }
        }
    }))
}

#[utoipa::path(
    post,
    path = "/v1/games/{id}/join",
    params(
        ("id" = String, Path, description = "Game ID in UUID format", format = "uuid")
    ),
    request_body = JoinGameRequest,
    responses(
        (status = 200, description = "Joined game successfully", body = GameDisplayDTO),
        (status = 400, description = "Cannot join game", body = InvalidCredentialsResponse),
        (status = 404, description = "Game not found", body = NotFoundResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Games"
)]
#[post("/{id}/join")]
pub async fn join_game(id: Path<Uuid>, payload: Json<JoinGameRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would add the player to the game
            // For now, we'll just return a mock response
            HttpResponse::Ok().json(json!({
                "message": "Joined game successfully",
                "data": {
                    "game": {
                        "id": id.into_inner(),
                        "status": "in_progress",
                        "player_id": payload.0.player_id
                    }
                }
            }))
        }
        Err(errors) => ApiError::ValidationError(errors).error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/games/{id}",
    params(
        ("id" = String, Path, description = "Game ID in UUID format", format = "uuid")
    ),
    responses(
        (status = 200, description = "Game abandoned successfully"),
        (status = 404, description = "Game not found", body = NotFoundResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Games"
)]
#[delete("/{id}")]
pub async fn abandon_game(id: Path<Uuid>) -> HttpResponse {
    // The real implementation would mark the game as abandoned
    // For now, we'll just return a mock response
    HttpResponse::Ok().json(json!({
        "message": "Game abandoned successfully",
        "data": {}
    }))
}