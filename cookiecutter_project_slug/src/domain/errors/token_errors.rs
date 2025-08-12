use crate::domain::error::CommonError;
use std::error::Error;
use std::fmt;

use super::response_code::ApiResponseCode;

#[derive(Debug)]
pub enum TokenError {
    RefreshTokenAlreadyExist,
    RefreshTokenDoesNotExists,
    TokenExpired,
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
        }
    }
}

impl From<TokenError> for CommonError {
    fn from(value: TokenError) -> Self {
        let code = match value {
            TokenError::RefreshTokenDoesNotExists => ApiResponseCode::NotFound,
            TokenError::RefreshTokenAlreadyExist => ApiResponseCode::Conflict,
            TokenError::TokenExpired => ApiResponseCode::Unauthorized,
        };

        CommonError {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for TokenError {}
