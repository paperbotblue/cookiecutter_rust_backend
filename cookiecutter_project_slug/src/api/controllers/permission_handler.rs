use std::str::FromStr;

use crate::api::dto::permission::{CreatePermissionDTO, PermissionDTO, UpdatePermissionDTO};
use crate::api::dto::wapper_uuid::UuidParam;
use crate::domain::error::{ApiError, CommonError};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::permission::PermissionService;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

pub async fn create_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    post_data: web::Json<CreatePermissionDTO>,
) -> Result<web::Json<PermissionDTO>, ApiError> {
    let permission = permission_service
        .create(post_data.into_inner().into())
        .await?;
    Ok(web::Json(permission.into()))
}

pub async fn update_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    post_data: web::Json<UpdatePermissionDTO>,
) -> Result<web::Json<PermissionDTO>, ApiError> {
    let permission = permission_service
        .update(post_data.into_inner().into())
        .await?;
    Ok(web::Json(permission.into()))
}

pub async fn list_permissions_handler(
    permission_service: web::Data<dyn PermissionService>,
    params: web::Query<PermissionQueryParams>,
) -> Result<web::Json<ResultPaging<PermissionDTO>>, ApiError> {
    let selection = permission_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    params: UuidParam,
) -> Result<web::Json<PermissionDTO>, ApiError> {
    let permission = permission_service.get(params.0).await?;
    Ok(web::Json(permission.into()))
}

pub async fn delete_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    params: UuidParam,
) -> Result<HttpResponse, ApiError> {
    permission_service.delete(params.0).await?;
    Ok(HttpResponse::NoContent().finish())
}
