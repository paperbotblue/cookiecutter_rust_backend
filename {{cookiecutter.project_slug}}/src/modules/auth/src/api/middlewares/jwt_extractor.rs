use std::sync::Arc;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web, Error, HttpMessage,
};

use crate::domain::services::refresh_token::RefreshTokenService;
use crate::domain::{errors::middleware_errors::MiddlewareError, services::role::RoleService};
use base::error::ApiError;

pub async fn check_permission_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    permission: &str,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token_service = get_service::<dyn RefreshTokenService>(&req)?;
    let role_service = get_service::<dyn RoleService>(&req)?;

    let token = token_service.extract_token(&req).map_err(ApiError::from)?;

    let claims = token_service.verify_jwt(&token).map_err(ApiError::from)?;

    let user_role = claims.role.clone();
    role_service
        .check_permission(user_role, permission.to_string())
        .await
        .map_err(ApiError::from)?;

    req.extensions_mut().insert(claims);

    next.call(req).await
}

fn get_service<T: 'static + Sync + Send + ?Sized>(
    req: &ServiceRequest,
) -> Result<Arc<T>, ApiError> {
    match req.app_data::<web::Data<T>>() {
        None => Err(MiddlewareError::InternalServerError("Unable to get service").into()),
        Some(service) => Ok(Arc::clone(service)),
    }
}
