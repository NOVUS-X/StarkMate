use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AiSuggestionRequest {
    #[validate(length(min = 10, message = "Must be a valid FEN string"))]
    #[schema(example = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    pub fen: String,
    
    #[validate(range(min = 1, max = 20, message = "Depth must be between 1 and 20"))]
    #[schema(example = 10)]
    pub depth: Option<u8>,
    
    #[validate(range(min = 1000, max = 60000, message = "Time limit must be between 1 and 60 seconds"))]
    #[schema(example = 5000)]
    pub time_limit_ms: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AiSuggestionResponse {
    #[schema(example = "e2e4")]
    pub best_move: String,
    
    #[schema(example = 0.3)]
    pub evaluation: f32,
    
    #[schema(example = 12)]
    pub depth: u8,
    
    pub principal_variation: Vec<String>,
    
    #[schema(example = 2345)]
    pub computation_time_ms: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct PositionAnalysisRequest {
    #[validate(length(min = 10, message = "Must be a valid FEN string"))]
    #[schema(example = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    pub fen: String,
    
    #[validate(range(min = 1, max = 25, message = "Depth must be between 1 and 25"))]
    #[schema(example = 15)]
    pub depth: u8,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PositionAnalysisResponse {
    #[schema(example = 0.3)]
    pub evaluation: f32,
    
    pub best_line: Vec<String>,
    
    pub alternatives: Vec<AlternativeMove>,
    
    #[schema(example = "Open Game")]
    pub position_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AlternativeMove {
    #[schema(example = "e2e4")]
    pub chess_move: String,
    
    #[schema(example = 0.25)]
    pub evaluation: f32,
}
