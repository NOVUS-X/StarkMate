use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 4, message = "Username must be at least 4 characters"))]
    #[schema(example = "chess_master")]
    pub username: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(example = "secure_password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
    
    #[schema(example = "Bearer")]
    pub token_type: String,
    
    #[schema(example = 3600)]
    pub expires_in: i32,
    
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    #[schema(value_type = String, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "chess_master")]
    pub username: String,
    
    #[schema(example = "chess@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 4, max = 20, message = "Username must be between 4 and 20 characters"))]
    #[schema(example = "chess_master")]
    pub username: String,
    
    #[validate(email(message = "Email must be valid"))]
    #[schema(example = "chess@example.com")]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(example = "secure_password123")]
    pub password: String,
    
    #[schema(example = "0x123abc...")]
    pub wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
    
    #[schema(example = "Bearer")]
    pub token_type: String,
    
    #[schema(example = 3600)]
    pub expires_in: i32,
}
