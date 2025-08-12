use crate::domain::error::CommonError;
use std::error::Error;
use std::fmt;

use super::response_code::ApiResponseCode;

#[derive(Debug)]
pub enum RolePermissionError {
    RolePermissionDoesNotExist,
    RolePermissionAlreadyExist,
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
        }
    }
}

impl From<RolePermissionError> for CommonError {
    fn from(value: RolePermissionError) -> Self {
        let code = match value {
            RolePermissionError::RolePermissionAlreadyExist => ApiResponseCode::Conflict,
            RolePermissionError::RolePermissionDoesNotExist => ApiResponseCode::NotFound,
        };

        CommonError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for RolePermissionError {}
