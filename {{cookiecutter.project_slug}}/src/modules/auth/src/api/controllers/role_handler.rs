use std::str::FromStr;

use crate::api::dto::role::{CreateRoleDTO, RoleDTO, UpdateRoleDTO};
use crate::domain::repositories::role::RoleQueryParams;
use crate::domain::services::role::RoleService;
use actix_web::{web, HttpResponse, Result};
use base::error::{ApiError, ApiResponse};
use base::result_paging::ResultPaging;
use uuid::Uuid;

pub async fn create_role_handler(
    role_service: web::Data<dyn RoleService>,
    post_data: web::Json<CreateRoleDTO>,
) -> Result<ApiResponse<RoleDTO>, ApiError> {
    let role = role_service.create(post_data.into_inner().into()).await?;
    Ok(ApiResponse(role.into()))
}

pub async fn update_role_handler(
    role_service: web::Data<dyn RoleService>,
    post_data: web::Json<UpdateRoleDTO>,
) -> Result<ApiResponse<RoleDTO>, ApiError> {
    let role = role_service.update(post_data.into_inner().into()).await?;
    Ok(ApiResponse(role.into()))
}

pub async fn list_roles_handler(
    role_service: web::Data<dyn RoleService>,
    params: web::Query<RoleQueryParams>,
) -> Result<ApiResponse<ResultPaging<RoleDTO>>, ApiError> {
    let selection = role_service.list(params.into_inner()).await?;
    Ok(ApiResponse(selection.map(|item| item.into())))
}

pub async fn get_role_handler(
    role_service: web::Data<dyn RoleService>,
    params: web::Path<String>,
) -> Result<ApiResponse<RoleDTO>, ApiError> {
    let role = role_service.get(Uuid::from_str(&params)?).await?;
    Ok(ApiResponse(role.into()))
}

pub async fn delete_role_handler(
    role_service: web::Data<dyn RoleService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    role_service.delete(Uuid::from_str(&params)?).await?;
    Ok(HttpResponse::NoContent().finish())
}
