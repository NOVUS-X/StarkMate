use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde_json::json;

mod chess960;
use chess960::{Chess960PositionsManager, Chess960Generator, FenValidator};

// Initialize positions manager globally
lazy_static::lazy_static! {
    static ref POSITIONS_MANAGER: std::sync::Mutex<Chess960PositionsManager> = {
        let mut manager = Chess960PositionsManager::new();
        manager.load_all_positions();
        std::sync::Mutex::new(manager)
    };
}

/// Get all Chess960 positions
async fn get_all_positions() -> Result<HttpResponse> {
    let manager = POSITIONS_MANAGER.lock().unwrap();
    let positions = manager.get_all_positions();
    Ok(HttpResponse::Ok().json(positions))
}

/// Get specific Chess960 position by number
async fn get_position(path: web::Path<u16>) -> Result<HttpResponse> {
    let position_number = path.into_inner();
    let manager = POSITIONS_MANAGER.lock().unwrap();
    
    match manager.get_position(position_number) {
        Some(position) => Ok(HttpResponse::Ok().json(position)),
        None => Ok(HttpResponse::NotFound().json(json!({
            "error": "Position not found",
            "message": format!("Position {} does not exist. Valid range: 1-960", position_number)
        })))
    }
}

/// Get random Chess960 position
async fn get_random_position() -> Result<HttpResponse> {
    let position = Chess960Generator::get_random_position();
    Ok(HttpResponse::Ok().json(position))
}

/// Validate Chess960 FEN
async fn validate_fen(fen_data: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    if let Some(fen) = fen_data.get("fen").and_then(|v| v.as_str()) {
        match FenValidator::validate_chess960_fen(fen) {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({
                "valid": true,
                "message": "FEN is valid Chess960 position"
            }))),
            Err(error) => Ok(HttpResponse::BadRequest().json(json!({
                "valid": false,
                "error": error
            })))
        }
    } else {
        Ok(HttpResponse::BadRequest().json(json!({
            "error": "Missing 'fen' field in request body"
        })))
    }
}

// Configure routes
pub fn configure_chess960_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/chess960")
            .route("/positions", web::get().to(get_all_positions))
            .route("/positions/{position_number}", web::get().to(get_position))
            .route("/random", web::get().to(get_random_position))
            .route("/validate", web::post().to(validate_fen))
    );
}