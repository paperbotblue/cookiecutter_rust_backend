use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use uuid::Error as UuidError;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
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
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(&self.message)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
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

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self {
            message: value.to_string(),
            code: 500,
        }
    }
}
