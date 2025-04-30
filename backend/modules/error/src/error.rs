use core::fmt;

use actix_web::{Error, HttpRequest, HttpResponse, error::JsonPayloadError};
use sea_orm::DbErr;
use serde_json::json;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug)]
pub enum ApiError {
    InvalidCredentials,
    DatabaseError(DbErr),
    NotFound(String),
    ValidationError(ValidationErrors),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InvalidCredentials => write!(f, "Invalid credentials"),
            ApiError::NotFound(v) => write!(f, "{} not found", v),
            ApiError::DatabaseError(err) => write!(f, "Database error {}", err.to_string()),
            ApiError::ValidationError(errs) => {
                let mut s = String::new();
                for (_, error_kind) in errs.errors() {
                    match error_kind {
                        ValidationErrorsKind::Field(field) => s.push_str(format!("{}. ", field[0].message.clone().unwrap()).as_str()),
                        _ => todo!()
                    }
                }
                write!(f,"{}",s)
            }
        }
    }
}

impl ApiError {
    pub fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApiError::InvalidCredentials => HttpResponse::BadRequest().json(json!({
                "error": self.to_string(),
                "code": 400
            })),
            ApiError::NotFound(_) => HttpResponse::NotFound().json(json!({
                "error": self.to_string(),
                "code": 404
            })),
            ApiError::DatabaseError(_) => HttpResponse::InternalServerError().json(json!({
                "error": self.to_string(),
                "code":500
            })),
            ApiError::ValidationError(_) => HttpResponse::BadRequest().json(json!({
                "error": self.to_string(),
                "code":400
            })),
        }
    }
}

pub fn custom_json_error(err: JsonPayloadError, _: &HttpRequest) -> Error {
    let error_response = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(json!({
            "error":"Invalid Content-Type. Expecting application/json",
            "code": 415
        })),
        // JsonPayloadError::Deserialize(err) => HttpResponse::BadRequest().json(json!({
        //     "error":err.to_string()
        // })),
        _ => HttpResponse::BadRequest().json(json!({
            "error":err.to_string(),
            "body":{}
        })),
    };

    actix_web::error::InternalError::from_response(err, error_response).into()
}
