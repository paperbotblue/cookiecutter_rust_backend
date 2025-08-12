use super::response_code::ApiResponseCode;
use crate::domain::error::CommonError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PermissionError {
    PermissionAlreadyExists,
    PermissionDoesNotExist,
    PermissionNotAuthorised,
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
        }
    }
}

impl From<PermissionError> for CommonError {
    fn from(value: PermissionError) -> Self {
        let code = match value {
            PermissionError::PermissionNotAuthorised => ApiResponseCode::Forbidden,
            PermissionError::PermissionDoesNotExist => ApiResponseCode::NotFound,
            PermissionError::PermissionAlreadyExists => ApiResponseCode::Conflict,
        };

        CommonError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for PermissionError {}
