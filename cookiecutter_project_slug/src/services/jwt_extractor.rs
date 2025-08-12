use std::sync::Arc;

use actix_web::{dev::ServiceRequest, http::header::AUTHORIZATION, web};
use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    errors::middleware_errors::MiddlewareError,
    services::{jwt_extractor::JwtExtractorService, role::RoleService, token::TokenService},
};

#[derive(Clone)]
pub struct JwtExtractorServiceImpl {}

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
