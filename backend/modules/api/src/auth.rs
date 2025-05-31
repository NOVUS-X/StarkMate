use actix_web::{
    HttpResponse, post,
    web::Json,
};
use dto::{
    auth::{LoginRequest, LoginResponse, RegisterRequest, RefreshTokenRequest, TokenResponse},
    responses::{InvalidCredentialsResponse, ValidationErrorResponse},
};
use error::error::ApiError;
use serde_json::json;
use validator::Validate;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = InvalidCredentialsResponse)
    ),
    tag = "Authentication"
)]
#[post("/login")]
pub async fn login(payload: Json<LoginRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would verify credentials
            // For now, we'll just return a mock response
            HttpResponse::Ok().json(json!({
                "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "token_type": "Bearer",
                "expires_in": 3600,
                "user": {
                    "id": Uuid::new_v4(),
                    "username": payload.0.username,
                    "email": "user@example.com"
                }
            }))
        }
        Err(errors) => ApiError::ValidationError(errors).error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = LoginResponse),
        (status = 400, description = "Invalid request parameters", body = InvalidCredentialsResponse),
        (status = 409, description = "User already exists", body = InvalidCredentialsResponse)
    ),
    tag = "Authentication"
)]
#[post("/register")]
pub async fn register(payload: Json<RegisterRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would create a user
            // For now, we'll just return a mock response
            HttpResponse::Created().json(json!({
                "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "token_type": "Bearer",
                "expires_in": 3600,
                "user": {
                    "id": Uuid::new_v4(),
                    "username": payload.0.username,
                    "email": payload.0.email
                }
            }))
        }
        Err(errors) => ApiError::ValidationError(errors).error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenResponse),
        (status = 401, description = "Invalid refresh token", body = InvalidCredentialsResponse),
        (status = 400, description = "Validation error", body = ValidationErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/refresh")]
pub async fn refresh_token(payload: Json<RefreshTokenRequest>) -> HttpResponse {
    match payload.0.validate() {
        Ok(_) => {
            // The real implementation would verify the refresh token
            // For now, we'll just return a mock response
            HttpResponse::Ok().json(json!({
                "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "token_type": "Bearer",
                "expires_in": 3600
            }))
        }
        Err(errors) => {
            let error_strings: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(_, errs)| errs.iter().map(|err| err.message.clone().unwrap_or_default().to_string()))
                .collect();
            
            HttpResponse::BadRequest().json(ValidationErrorResponse {
                error: "Invalid refresh token format".to_string(),
                code: 400,
                details: Some(error_strings)
            })
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/protected/logout",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized", body = InvalidCredentialsResponse)
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "Authentication"
)]
#[post("/logout")]
pub async fn logout() -> HttpResponse {
    // The real implementation would invalidate the token
    // For now, we'll just return a mock response
    HttpResponse::Ok().json(json!({
        "message": "Logout successful"
    }))
}
