use actix_web::{
    HttpResponse, post,
    web::Json,
};
use dto::{
    ai::{AiSuggestionRequest, AiSuggestionResponse, PositionAnalysisRequest, PositionAnalysisResponse},
    responses::ValidationErrorResponse,
};
use error::error::ApiError;
use serde_json::json;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/v1/ai/suggest",
    request_body = AiSuggestionRequest,
    responses(
        (status = 200, description = "AI suggestion generated", body = AiSuggestionResponse),
        (status = 400, description = "Invalid FEN position", body = ValidationErrorResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "AI"
)]
#[post("/suggest")]
pub async fn get_ai_suggestion(payload: Json<AiSuggestionRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would call the chess engine
            // For now, we'll just return a mock response
            HttpResponse::Ok().json(json!({
                "best_move": "e2e4",
                "evaluation": 0.3,
                "depth": payload.0.depth.unwrap_or(10),
                "principal_variation": ["e2e4", "e7e5", "Ng1f3"],
                "computation_time_ms": 2345
            }))
        }
        Err(errors) => {
            let error_strings: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(_, errs)| errs.iter().map(|err| err.message.clone().unwrap_or_default().to_string()))
                .collect();
            
            HttpResponse::BadRequest().json(ValidationErrorResponse {
                error: "Invalid FEN position or parameters".to_string(),
                code: 400,
                details: Some(error_strings)
            })
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/ai/analyze",
    request_body = PositionAnalysisRequest,
    responses(
        (status = 200, description = "Position analysis completed", body = PositionAnalysisResponse),
        (status = 400, description = "Invalid FEN position", body = ValidationErrorResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "AI"
)]
#[post("/analyze")]
pub async fn analyze_position(payload: Json<PositionAnalysisRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would analyze the position
            // For now, we'll just return a mock response
            HttpResponse::Ok().json(json!({
                "evaluation": 0.3,
                "best_line": ["e2e4", "e7e5", "Ng1f3", "Nb8c6"],
                "alternatives": [
                    {
                        "chess_move": "d2d4",
                        "evaluation": 0.25
                    },
                    {
                        "chess_move": "c2c4",
                        "evaluation": 0.20
                    }
                ],
                "position_type": "Open Game"
            }))
        }
        Err(errors) => {
            let error_strings: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(_, errs)| errs.iter().map(|err| err.message.clone().unwrap_or_default().to_string()))
                .collect();
            
            HttpResponse::BadRequest().json(ValidationErrorResponse {
                error: "Invalid FEN position or parameters".to_string(),
                code: 400,
                details: Some(error_strings)
            })
        }
    }
}
