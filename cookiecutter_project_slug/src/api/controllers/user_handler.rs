use std::str::FromStr;

use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::user::{CreateUserDTO, UpdateUserDTO, UserDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::services::user::UserService;

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<UserDTO>, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(user.into()))
}

pub async fn update_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<UpdateUserDTO>,
) -> Result<web::Json<UserDTO>, ApiError> {
    let user = user_service.update(post_data.into_inner().into()).await?;
    Ok(web::Json(user.into()))
}

pub async fn list_users_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Query<UserQueryParams>,
) -> Result<web::Json<ResultPaging<UserDTO>>, ApiError> {
    let selection = user_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_user_handler(
    user_service: web::Data<dyn UserService>,
    params: String,
) -> Result<web::Json<UserDTO>, ApiError> {
    let user = user_service.get(Uuid::from_str(&params)?).await?;
    Ok(web::Json(user.into()))
}

pub async fn delete_user_handler(
    user_service: web::Data<dyn UserService>,
    params: String,
) -> Result<HttpResponse, ApiError> {
    user_service.delete(Uuid::from_str(&params)?).await?;
    Ok(HttpResponse::NoContent().finish())
}
