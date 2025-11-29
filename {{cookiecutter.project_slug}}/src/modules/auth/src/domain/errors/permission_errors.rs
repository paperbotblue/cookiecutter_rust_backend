use base::error::{ApiError, RepositoryError};
use base::response_code::ApiResponseCode;
use shared::file_handler::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PermissionError {
    PermissionAlreadyExists,
    PermissionDoesNotExist,
    PermissionNotAuthorised,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionError::PermissionDoesNotExist => {
                write!(f, "Permission Fetching Error: Does Not Exist")
            }

            PermissionError::PermissionAlreadyExists => {
                write!(f, "Permission Creation Error: Already Exists")
            }

            PermissionError::PermissionNotAuthorised => {
                write!(f, "Permission Not Authorised")
            }

            PermissionError::InternalServerError(error) => {
                write!(
                    f,
                    "Internal Server Error(MiddlewareError): {}",
                    error.message
                )
            }
        }
    }
}

impl From<PermissionError> for ApiError {
    fn from(value: PermissionError) -> Self {
        let code = match value {
            PermissionError::PermissionNotAuthorised => ApiResponseCode::Forbidden,
            PermissionError::PermissionDoesNotExist => ApiResponseCode::NotFound,
            PermissionError::PermissionAlreadyExists => ApiResponseCode::Conflict,
            PermissionError::InternalServerError(ref e) => {
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

impl Error for PermissionError {}
