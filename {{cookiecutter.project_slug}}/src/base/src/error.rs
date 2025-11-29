use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use uuid::Error as UuidError;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize>(pub T);
impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(json!({
            "data": self.0,
            "msg": "",
            "success": true,
            "statusCode": 200
        }))
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl ApiError {
    pub fn new(message: impl Into<String>, code: u16) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap()
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "data": {},
            "msg": self.message,
            "success": false,
            "statusCode": self.code
        }))
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl RepositoryError {
    pub fn new(err: String) -> Self {
        Self { message: err }
    }
}

impl From<RepositoryError> for ApiError {
    fn from(value: RepositoryError) -> Self {
        Self {
            message: value.message,
            code: 500,
        }
    }
}

impl From<UuidError> for ApiError {
    fn from(value: UuidError) -> Self {
        Self {
            message: value.to_string(),
            code: 400,
        }
    }
}

impl From<sqlx::error::Error> for RepositoryError {
    fn from(value: sqlx::error::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for RepositoryError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
