use super::response_code::ApiResponseCode;
use crate::domain::error::ApiError;
use crate::utils::append_to_file::save_error;
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

impl From<MiddlewareError<'_>> for ApiError {
    fn from(value: MiddlewareError) -> Self {
        let code = match value {
            MiddlewareError::AuthHeaderDoesNotExist => ApiResponseCode::Forbidden,
            MiddlewareError::InvalidAuthHeader => ApiResponseCode::Forbidden,
            MiddlewareError::InvalidTokenFormat => ApiResponseCode::Forbidden,
            MiddlewareError::UnableToCallNext => ApiResponseCode::InternalServerError,
            MiddlewareError::InternalServerError(e) => {
                save_error(e);
                ApiResponseCode::InternalServerError
            }
        };

        ApiError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for MiddlewareError<'_> {}
