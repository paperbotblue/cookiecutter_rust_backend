use base::error::{ApiError, RepositoryError};
use base::response_code::ApiResponseCode;
use shared::file_handler::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UserError {
    UserAlreadyExists,
    UserDoesNotExist,
    UserNotAuthorised,
    UserOtpAttemptsExceeded,
    UserOtpExpired,
    UserOtpNotExpired,
    UserOtpIncorrect,
    InvalidCredentials,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::UserDoesNotExist => {
                write!(f, "User Fetching Error: Does Not Exist")
            }

            UserError::UserAlreadyExists => {
                write!(f, "User Creation Error: Already Exists")
            }

            UserError::UserNotAuthorised => {
                write!(f, "User Not Authorised")
            }

            UserError::UserOtpExpired => {
                write!(f, "User Otp Expired")
            }

            UserError::UserOtpNotExpired => {
                write!(
                    f,
                    "User Is In Cooldown Period Please Try Again After Some Time For Generating Otp"
                )
            }

            UserError::UserOtpIncorrect => {
                write!(f, "User Otp Incorrect")
            }

            UserError::UserOtpAttemptsExceeded => {
                write!(f, "User Otp Retry Attempts Exceeded")
            }

            UserError::InvalidCredentials => {
                write!(f, "Invalid Credentials")
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
            UserError::UserAlreadyExists => ApiResponseCode::Conflict,
            UserError::UserOtpAttemptsExceeded => ApiResponseCode::BadRequest,
            UserError::UserOtpExpired => ApiResponseCode::BadRequest,
            UserError::UserOtpNotExpired => ApiResponseCode::BadRequest,
            UserError::UserOtpIncorrect => ApiResponseCode::BadRequest,
            UserError::InvalidCredentials => ApiResponseCode::Forbidden,
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
