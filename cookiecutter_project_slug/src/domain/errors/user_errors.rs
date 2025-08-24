use super::response_code::ApiResponseCode;
use crate::domain::error::{ApiError, RepositoryError};
use crate::utils::append_to_file::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UserError {
    UserAlreadyExistsEmail,
    UserAlreadyExistsPhoneNumber,
    UserDoesNotExist,
    UserNotAuthorised,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::UserDoesNotExist => {
                write!(f, "User Fetching Error: Does Not Exist")
            }

            UserError::UserAlreadyExistsEmail => {
                write!(f, "User Creation Error: Already Exists With This Email")
            }

            UserError::UserAlreadyExistsPhoneNumber => {
                write!(
                    f,
                    "User Creation Error: Already Exists With This Phone Number"
                )
            }

            UserError::UserNotAuthorised => {
                write!(f, "User Not Authorised")
            }

            UserError::InternalServerError(error) => {
                write!(f, "Internal Server Error(UserError): {}", &error.message)
            }
        }
    }
}

impl From<UserError> for ApiError {
    fn from(value: UserError) -> Self {
        let code = match value {
            UserError::UserNotAuthorised => ApiResponseCode::Forbidden,
            UserError::UserDoesNotExist => ApiResponseCode::NotFound,
            UserError::UserAlreadyExistsEmail => ApiResponseCode::Conflict,
            UserError::UserAlreadyExistsPhoneNumber => ApiResponseCode::Conflict,
            UserError::InternalServerError(ref e) => {
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

impl Error for UserError {}
