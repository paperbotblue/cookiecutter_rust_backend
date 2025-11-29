use base::error::{ApiError, RepositoryError};
use shared::file_handler::save_error;
use std::error::Error;
use std::fmt;

use base::response_code::ApiResponseCode;

#[derive(Debug)]
pub enum RoleError {
    RoleDoesNotExists,
    RoleAlreadyExist,
    RoleNotAuthorised,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for RoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleError::RoleDoesNotExists => {
                write!(f, "Role Fetching Error: Does Not Exists")
            }

            RoleError::RoleNotAuthorised => {
                write!(f, "Role is not authorised")
            }

            RoleError::RoleAlreadyExist => {
                write!(f, "Role Creation Error: Already Exist")
            }
            RoleError::InternalServerError(error) => {
                write!(f, "Internal Server Error(RoleError): {}", &error.message)
            }
        }
    }
}

impl From<RoleError> for ApiError {
    fn from(value: RoleError) -> Self {
        let code = match value {
            RoleError::RoleAlreadyExist => ApiResponseCode::Conflict,
            RoleError::RoleDoesNotExists => ApiResponseCode::NotFound,
            RoleError::RoleNotAuthorised => ApiResponseCode::Forbidden,
            RoleError::InternalServerError(ref e) => {
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

impl Error for RoleError {}
