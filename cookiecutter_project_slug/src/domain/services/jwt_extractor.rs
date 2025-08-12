use std::sync::Arc;

use actix_web::dev::ServiceRequest;
use async_trait::async_trait;

use crate::domain::error::CommonError;

use super::{role::RoleService, token::TokenService};

#[async_trait]
pub trait JwtExtractorService: 'static + Sync + Send {
    fn get_token(&self, req: &ServiceRequest) -> Result<String, CommonError>;
}
