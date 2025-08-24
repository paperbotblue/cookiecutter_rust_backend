use crate::domain::error::{ApiError, RepositoryError};
use crate::utils::append_to_file::save_error;
use std::error::Error;
use std::fmt;

use super::response_code::ApiResponseCode;

#[derive(Debug)]
pub enum RolePermissionError {
    RolePermissionDoesNotExist,
    RolePermissionAlreadyExist,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for RolePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RolePermissionError::RolePermissionDoesNotExist => {
                write!(f, "RolePermission Fetching Error: Does Not Exist")
            }
            RolePermissionError::RolePermissionAlreadyExist => {
                write!(f, "RolePermission Creation Error: Already Exists")
            }

            RolePermissionError::InternalServerError(error) => {
                write!(
                    f,
                    "Internal Server Error(RolePermissionError): {}",
                    error.message
                )
            }
        }
    }
}

impl From<RolePermissionError> for ApiError {
    fn from(value: RolePermissionError) -> Self {
        let code = match value {
            RolePermissionError::RolePermissionAlreadyExist => ApiResponseCode::Conflict,
            RolePermissionError::RolePermissionDoesNotExist => ApiResponseCode::NotFound,
            RolePermissionError::InternalServerError(ref e) => {
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

impl Error for RolePermissionError {}
