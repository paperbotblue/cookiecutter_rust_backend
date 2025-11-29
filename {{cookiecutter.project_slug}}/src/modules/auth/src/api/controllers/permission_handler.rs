use std::str::FromStr;

use crate::api::dto::permission::{CreatePermissionDTO, PermissionDTO, UpdatePermissionDTO};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::services::permission::PermissionService;
use actix_web::{web, HttpResponse, Result};
use base::error::{ApiError, ApiResponse};
use base::result_paging::ResultPaging;
use uuid::Uuid;

pub async fn create_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    post_data: web::Json<CreatePermissionDTO>,
) -> Result<ApiResponse<PermissionDTO>, ApiError> {
    let permission = permission_service
        .create(post_data.into_inner().into())
        .await?;
    Ok(ApiResponse(permission.into()))
}

pub async fn update_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    post_data: web::Json<UpdatePermissionDTO>,
) -> Result<ApiResponse<PermissionDTO>, ApiError> {
    let permission = permission_service
        .update(post_data.into_inner().into())
        .await?;
    Ok(ApiResponse(permission.into()))
}

pub async fn list_permissions_handler(
    permission_service: web::Data<dyn PermissionService>,
    params: web::Query<PermissionQueryParams>,
) -> Result<ApiResponse<ResultPaging<PermissionDTO>>, ApiError> {
    let selection = permission_service.list(params.into_inner()).await?;
    Ok(ApiResponse(selection.map(|item| item.into())))
}

pub async fn get_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    params: web::Path<Uuid>,
) -> Result<ApiResponse<PermissionDTO>, ApiError> {
    let permission = permission_service.get(params.into_inner()).await?;
    Ok(ApiResponse(permission.into()))
}

pub async fn delete_permission_handler(
    permission_service: web::Data<dyn PermissionService>,
    params: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    permission_service.delete(params.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
