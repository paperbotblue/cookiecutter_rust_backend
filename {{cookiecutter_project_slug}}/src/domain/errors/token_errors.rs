use crate::domain::error::{ApiError, RepositoryError};
use crate::utils::append_to_file::save_error;
use std::error::Error;
use std::fmt;

use super::response_code::ApiResponseCode;

#[derive(Debug)]
pub enum TokenError {
    RefreshTokenAlreadyExist,
    RefreshTokenDoesNotExists,
    TokenExpired,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::RefreshTokenAlreadyExist => {
                write!(f, "Refresh Token Creation Error: Already Exist")
            }

            TokenError::RefreshTokenDoesNotExists => {
                write!(f, "Refresh Token fetching Error: Does Not Exists")
            }
            TokenError::TokenExpired => {
                write!(f, "Token Expired")
            }

            TokenError::InternalServerError(error) => {
                write!(
                    f,
                    "Internal Server Error(MiddlewareError): {}",
                    error.message
                )
            }
        }
    }
}

impl From<TokenError> for ApiError {
    fn from(value: TokenError) -> Self {
        let code = match value {
            TokenError::RefreshTokenDoesNotExists => ApiResponseCode::NotFound,
            TokenError::RefreshTokenAlreadyExist => ApiResponseCode::Conflict,
            TokenError::TokenExpired => ApiResponseCode::Unauthorized,
            TokenError::InternalServerError(ref e) => {
                save_error(&e.message);
                ApiResponseCode::InternalServerError
            }
        };

        Self {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for TokenError {}
