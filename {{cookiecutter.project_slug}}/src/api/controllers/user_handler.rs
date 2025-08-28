use std::str::FromStr;

use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::user::{CreateUserDTO, UpdateUserDTO, UserDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::services::refresh_token::RefreshTokenService;
use crate::domain::services::user::UserService;

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<HttpResponse, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;

    let raw_token = Uuid::new_v4();
    refresh_token_service
        .create_new_user_refresh_token(user.id, raw_token)
        .await?;
    let jwt_token = refresh_token_service
        .create_user_jwt_token(user.id, "User".to_string())
        .await?;

    let cookie = refresh_token_service.build_refresh_token_cookie(raw_token.to_string())?;

    let res = HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
        "access_token": jwt_token
    }));
    Ok(res)
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
