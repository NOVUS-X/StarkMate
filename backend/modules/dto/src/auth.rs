use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;
use once_cell::sync::Lazy;
use regex::Regex;

// Define a regex for strong password validation
// Requires at least one uppercase, one lowercase, one digit, and one special character
static STRONG_PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap()
});

// Define a regex for Ethereum wallet address validation
// Requires 0x prefix followed by 40 hex characters
static WALLET_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap()
});

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 4, message = "Username must be at least 4 characters"))]
    #[schema(example = "chess_master")]
    pub username: String,
    
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        regex(
            path = "STRONG_PASSWORD_REGEX",
            message = "Password must contain at least one uppercase letter, one lowercase letter, one digit, and one special character"
        )
    )]
    #[schema(example = "Secure_password123!")]
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
    
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        regex(
            path = "STRONG_PASSWORD_REGEX",
            message = "Password must contain at least one uppercase letter, one lowercase letter, one digit, and one special character"
        )
    )]
    #[schema(example = "Secure_password123!")]
    pub password: String,
    
    #[validate(regex(
        path = "WALLET_ADDRESS_REGEX",
        message = "Wallet address must be a valid 0x-prefixed 40 hex character string"
    ))]
    #[schema(example = "0x1234567890abcdef1234567890abcdef12345678")]
    pub wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 10, message = "Refresh token is required and must be valid"))]
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
