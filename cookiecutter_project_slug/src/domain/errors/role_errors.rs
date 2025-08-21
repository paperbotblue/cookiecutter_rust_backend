use crate::domain::error::CommonError;
use crate::utils::append_to_file::append_to_file;
use std::error::Error;
use std::fmt;

use super::response_code::ApiResponseCode;

#[derive(Debug)]
pub enum RoleError {
    RoleDoesNotExists,
    RoleAlreadyExist,
    RoleNotAuthorised,
    InternalServerError(String),
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
                write!(f, "Internal Server Error(MiddlewareError): {}", error)
            }
        }
    }
}

impl From<RoleError> for CommonError {
    fn from(value: RoleError) -> Self {
        let code = match value {
            RoleError::RoleAlreadyExist => ApiResponseCode::Conflict,
            RoleError::RoleDoesNotExists => ApiResponseCode::NotFound,
            RoleError::RoleNotAuthorised => ApiResponseCode::Forbidden,
            RoleError::InternalServerError(ref e) => {
                append_to_file("../../../error_logs.txt", e);
                ApiResponseCode::InternalServerError
            }
        };

        CommonError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for RoleError {}
