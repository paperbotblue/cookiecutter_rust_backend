use base::error::{ApiError, RepositoryError};
use base::response_code::ApiResponseCode;
use shared::file_handler::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RefreshTokenError {
    RefreshTokenAlreadyExists,
    RefreshTokenDoesNotExist,
    JwtTokenDoesNotExist,
    RefreshTokenNotAuthorised,
    TokenExpired,
    InvalidUserId,
    InvalidTokenFormat,
    InvalidToken,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for RefreshTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefreshTokenError::RefreshTokenDoesNotExist => {
                write!(f, "RefreshToken Fetching Error: Does Not Exist")
            }

            RefreshTokenError::JwtTokenDoesNotExist => {
                write!(f, "JwtToken Fetching Error: Does Not Exist")
            }

            RefreshTokenError::RefreshTokenAlreadyExists => {
                write!(f, "RefreshToken Creation Error: Already Exists")
            }

            RefreshTokenError::RefreshTokenNotAuthorised => {
                write!(f, "RefreshToken Not Authorised")
            }

            RefreshTokenError::TokenExpired => {
                write!(f, "TokenExpired")
            }

            RefreshTokenError::InvalidUserId => {
                write!(f, "Invalid User Id")
            }
            RefreshTokenError::InvalidTokenFormat => {
                write!(f, "Invalid Token Format")
            }

            RefreshTokenError::InvalidToken => {
                write!(f, "Invalid Token")
            }

            RefreshTokenError::InternalServerError(error) => {
                write!(
                    f,
                    "Internal Server Error(RefreshTokenError): {}",
                    &error.message
                )
            }
        }
    }
}

impl From<RefreshTokenError> for ApiError {
    fn from(value: RefreshTokenError) -> Self {
        let code = match value {
            RefreshTokenError::RefreshTokenNotAuthorised => ApiResponseCode::Forbidden,
            RefreshTokenError::RefreshTokenDoesNotExist => ApiResponseCode::NotFound,
            RefreshTokenError::JwtTokenDoesNotExist => ApiResponseCode::BadRequest,
            RefreshTokenError::RefreshTokenAlreadyExists => ApiResponseCode::Conflict,
            RefreshTokenError::TokenExpired => ApiResponseCode::Forbidden,
            RefreshTokenError::InvalidUserId => ApiResponseCode::BadRequest,
            RefreshTokenError::InvalidTokenFormat => ApiResponseCode::BadRequest,
            RefreshTokenError::InvalidToken => ApiResponseCode::BadRequest,
            RefreshTokenError::InternalServerError(ref e) => {
                save_error(&e.message);
                ApiResponseCode::InternalServerError
            }
        };

        ApiError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for RefreshTokenError {}
