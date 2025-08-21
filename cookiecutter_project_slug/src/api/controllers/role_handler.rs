use crate::api::dto::role::{CreateRoleDTO, RoleDTO, UpdateRoleDTO};
use crate::api::dto::wapper_uuid::UuidParam;
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::role::RoleQueryParams;
use crate::domain::services::role::RoleService;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

pub async fn create_role_handler(
    role_service: web::Data<dyn RoleService>,
    post_data: web::Json<CreateRoleDTO>,
) -> Result<web::Json<RoleDTO>, ApiError> {
    let role = role_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(role.into()))
}

pub async fn update_role_handler(
    role_service: web::Data<dyn RoleService>,
    post_data: web::Json<UpdateRoleDTO>,
) -> Result<web::Json<RoleDTO>, ApiError> {
    let role = role_service.update(post_data.into_inner().into()).await?;
    Ok(web::Json(role.into()))
}

pub async fn list_roles_handler(
    role_service: web::Data<dyn RoleService>,
    params: web::Query<RoleQueryParams>,
) -> Result<web::Json<ResultPaging<RoleDTO>>, ApiError> {
    let selection = role_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_role_handler(
    role_service: web::Data<dyn RoleService>,
    params: UuidParam,
) -> Result<web::Json<RoleDTO>, ApiError> {
    let role = role_service.get(params.0).await?;
    Ok(web::Json(role.into()))
}

pub async fn delete_role_handler(
    role_service: web::Data<dyn RoleService>,
    params: UuidParam,
) -> Result<HttpResponse, ApiError> {
    role_service.delete(params.0).await?;
    Ok(HttpResponse::NoContent().finish())
}
