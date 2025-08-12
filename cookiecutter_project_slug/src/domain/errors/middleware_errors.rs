use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

use super::response_code::ApiResponseCode;
use super::token_errors::TokenError;
use crate::domain::error::CommonError;
use crate::utils::append_to_file::append_to_file;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MiddlewareError<'a> {
    AuthHeaderDoesNotExist,
    InvalidAuthHeader,
    UnableToCallNext,
    InvalidTokenFormat,
    InternalServerError(&'a str),
}

// Implement `Display`
impl fmt::Display for MiddlewareError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MiddlewareError::InvalidAuthHeader => {
                write!(f, "Invalid Auth Header")
            }

            MiddlewareError::InvalidTokenFormat => {
                write!(f, "Invalid token format")
            }

            MiddlewareError::UnableToCallNext => {
                write!(f, "Unable to call next")
            }

            MiddlewareError::AuthHeaderDoesNotExist => {
                write!(f, "Auth header does not exist")
            }

            MiddlewareError::InternalServerError(error) => {
                write!(f, "Internal Server Error(MiddlewareError): {}", error)
            }
        }
    }
}

impl From<MiddlewareError<'_>> for CommonError {
    fn from(value: MiddlewareError) -> Self {
        let code = match value {
            MiddlewareError::AuthHeaderDoesNotExist => ApiResponseCode::Forbidden,
            MiddlewareError::InvalidAuthHeader => ApiResponseCode::Forbidden,
            MiddlewareError::InvalidTokenFormat => ApiResponseCode::Forbidden,
            MiddlewareError::UnableToCallNext => ApiResponseCode::InternalServerError,
            MiddlewareError::InternalServerError(e) => {
                append_to_file("../../../error_logs.txt", &e);
                ApiResponseCode::InternalServerError
            }
        };

        CommonError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for MiddlewareError<'_> {}
