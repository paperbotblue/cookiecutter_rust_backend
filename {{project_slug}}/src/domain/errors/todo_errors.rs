use super::response_code::ApiResponseCode;
use crate::domain::error::ApiError;
use crate::utils::append_to_file::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TodoError {
    TodoAlreadyExists,
    TodoDoesNotExist,
    TodoNotAuthorised,
    InternalServerError(String),
}

// Implement `Display`
impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::TodoDoesNotExist => {
                write!(f, "Todo Fetching Error: Does Not Exist")
            }

            TodoError::TodoAlreadyExists => {
                write!(f, "Todo Creation Error: Already Exists")
            }

            TodoError::TodoNotAuthorised => {
                write!(f, "Todo Not Authorised")
            }

            TodoError::InternalServerError(error) => {
                write!(f, "Internal Server Error(MiddlewareError): {}", error)
            }
        }
    }
}

impl From<TodoError> for ApiError {
    fn from(value: TodoError) -> Self {
        let code = match value {
            TodoError::TodoNotAuthorised => ApiResponseCode::Forbidden,
            TodoError::TodoDoesNotExist => ApiResponseCode::NotFound,
            TodoError::TodoAlreadyExists => ApiResponseCode::Conflict,
            TodoError::InternalServerError(ref e) => {
                save_error(e);
                ApiResponseCode::InternalServerError
            }
        };

        Self {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for TodoError {}
