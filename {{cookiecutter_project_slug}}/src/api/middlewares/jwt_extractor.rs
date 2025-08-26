use std::sync::Arc;

use actix_web::http::header::AUTHORIZATION;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web, Error, HttpMessage,
};

use crate::domain::error::ApiError;
use crate::domain::{errors::middleware_errors::MiddlewareError, services::role::RoleService};

pub async fn check_permission_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    permission: &str,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token_service = get_service::<dyn TokenService>(&req)?;
    let role_service = get_service::<dyn RoleService>(&req)?;

    let token = get_token(&req).map_err(ApiError::from)?;

    let claims = token_service
        .verify_jwt_token(&token)
        .map_err(ApiError::from)?;

    token_service
        .expiration_check(&claims)
        .map_err(ApiError::from)?;

    let user_role = claims.permissions.clone();
    role_service
        .check_permission(&user_role, permission)
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

fn get_token(req: &ServiceRequest) -> Result<String, ApiError> {
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
