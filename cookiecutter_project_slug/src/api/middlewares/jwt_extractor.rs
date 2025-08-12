use std::sync::Arc;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    web, Error, HttpMessage,
};
use chrono::Utc;

use crate::domain::{
    error::{ApiError, CommonError},
    errors::{
        middleware_errors::MiddlewareError, permission_errors::PermissionError,
        token_errors::TokenError,
    },
    services::{jwt_extractor::JwtExtractorService, role::RoleService, token::TokenService},
};

pub async fn check_permission_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    permission: &str,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let jwt_extractor_service = get_service::<dyn JwtExtractorService>(&req)?;
    let token_service = get_service::<dyn TokenService>(&req)?;
    let role_service = get_service::<dyn RoleService>(&req)?;

    let token = jwt_extractor_service.get_token(&req)?;

    let claims = token_service.verify_jwt_token(&token)?;
    token_service.expiration_check(&claims)?;

    let user_role = claims.permissions.clone();
    role_service
        .check_permission(&user_role, permission)
        .await?;

    req.extensions_mut().insert(claims);

    Ok(next.call(req).await?)
}

fn get_service<T: 'static + Sync + Send + ?Sized>(
    req: &ServiceRequest,
) -> Result<Arc<T>, CommonError> {
    match req.app_data::<web::Data<T>>() {
        None => Err(MiddlewareError::InternalServerError("Unable to get service").into()),
        Some(service) => Ok(Arc::clone(service)),
    }
}
