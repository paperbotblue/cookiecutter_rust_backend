use actix_web::{dev::ServiceRequest, http::header::AUTHORIZATION};
use async_trait::async_trait;

use crate::domain::{
    error::CommonError, errors::middleware_errors::MiddlewareError,
    services::jwt_extractor::JwtExtractorService,
};

#[derive(Clone)]
pub struct JwtExtractorServiceImpl {}

impl Default for JwtExtractorServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl JwtExtractorServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl JwtExtractorService for JwtExtractorServiceImpl {
    fn get_token(&self, req: &ServiceRequest) -> Result<String, CommonError> {
        let auth_header = match req.headers().get(AUTHORIZATION) {
            Some(auth_header) => auth_header,
            None => return Err(MiddlewareError::AuthHeaderDoesNotExist.into()),
        };
        let auth_header = match auth_header.to_str() {
            Ok(auth_header) => auth_header,
            Err(err) => return Err(MiddlewareError::InternalServerError(&err.to_string()).into()),
        };

        if !auth_header.starts_with("Bearer ") {
            return Err(MiddlewareError::InvalidTokenFormat.into());
        }

        Ok(auth_header[7..].to_string())
    }
}
