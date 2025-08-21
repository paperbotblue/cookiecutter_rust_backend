use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u16,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl CommonError {
    pub fn new(message: impl Into<String>, code: u16) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}

impl ResponseError for CommonError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap()
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let response = json!({
            "status": self.status_code().as_u16(),
            "response": self.message
        });
        HttpResponse::build(self.status_code()).json(response)
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.0.code).unwrap()
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(&self.0)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl From<RepositoryError> for CommonError {
    fn from(value: RepositoryError) -> Self {
        Self {
            message: value.message,
            code: 500,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for CommonError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self {
            message: value.to_string(),
            code: 500,
        }
    }
}
