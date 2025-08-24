use super::response_code::ApiResponseCode;
use crate::domain::error::{ApiError, RepositoryError} ;
use crate::utils::append_to_file::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RefreshTokenError {
    RefreshTokenAlreadyExists,
    RefreshTokenDoesNotExist,
    RefreshTokenNotAuthorised,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for RefreshTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefreshTokenError::RefreshTokenDoesNotExist => {
                write!(f, "RefreshToken Fetching Error: Does Not Exist")
            }

            RefreshTokenError::RefreshTokenAlreadyExists => {
                write!(f, "RefreshToken Creation Error: Already Exists")
            }

            RefreshTokenError::RefreshTokenNotAuthorised => {
                write!(f, "RefreshToken Not Authorised")
            }

            RefreshTokenError::InternalServerError(error) => {
                write!(f, "Internal Server Error(RefreshTokenError): {}", &error.message)
            }
        }
    }
}

impl From<RefreshTokenError> for ApiError  {
    fn from(value: RefreshTokenError) -> Self {
        let code = match value {
            RefreshTokenError::RefreshTokenNotAuthorised => ApiResponseCode::Forbidden,
            RefreshTokenError::RefreshTokenDoesNotExist => ApiResponseCode::NotFound,
            RefreshTokenError::RefreshTokenAlreadyExists => ApiResponseCode::Conflict,
            RefreshTokenError::InternalServerError(ref e) => {
                save_error(&e.message);
                ApiResponseCode::InternalServerError
            }
        };

        ApiError  {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for RefreshTokenError {}
