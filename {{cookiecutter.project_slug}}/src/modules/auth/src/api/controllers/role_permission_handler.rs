use std::str::FromStr;

use crate::api::dto::role_permission::{CreateRolePermissionDTO, RolePermissionDTO};
use crate::domain::repositories::role_permission::RolePermissionQueryParams;
use crate::domain::services::role_permission::RolePermissionService;
use actix_web::{web, HttpResponse, Result};
use base::error::{ApiError, ApiResponse};
use base::result_paging::ResultPaging;
use uuid::Uuid;

pub async fn create_role_permission_handler(
    role_permission_service: web::Data<dyn RolePermissionService>,
    post_data: web::Json<CreateRolePermissionDTO>,
) -> Result<ApiResponse<RolePermissionDTO>, ApiError> {
    let role_permission = role_permission_service
        .create(post_data.into_inner().into())
        .await?;
    Ok(ApiResponse(role_permission.into()))
}

pub async fn list_role_permissions_handler(
    role_permission_service: web::Data<dyn RolePermissionService>,
    params: web::Query<RolePermissionQueryParams>,
) -> Result<ApiResponse<ResultPaging<RolePermissionDTO>>, ApiError> {
    let selection = role_permission_service.list(params.into_inner()).await?;
    Ok(ApiResponse(selection.map(|item| item.into())))
}

pub async fn get_role_permission_handler(
    role_permission_service: web::Data<dyn RolePermissionService>,
    params: web::Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<RolePermissionDTO>, ApiError> {
    let (role_id, permission_id) = params.into_inner();
    let role_permission = role_permission_service.get(role_id, permission_id).await?;
    Ok(ApiResponse(role_permission.into()))
}

pub async fn delete_role_permission_handler(
    role_permission_service: web::Data<dyn RolePermissionService>,
    params: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let (role_id, permission_id) = (Uuid::from_str(&params.0)?, Uuid::from_str(&params.1)?);
    role_permission_service
        .delete(role_id, permission_id)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
