use std::str::FromStr;

use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::refresh_token::{
    CreateRefreshTokenDTO, RefreshTokenDTO, UpdateRefreshTokenDTO,
};
use crate::domain::error::ApiError;
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::refresh_token::RefreshTokenService;

pub async fn create_refresh_token_handler(
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    post_data: web::Json<CreateRefreshTokenDTO>,
) -> Result<web::Json<RefreshTokenDTO>, ApiError> {
    let refresh_token = refresh_token_service
        .create(post_data.into_inner().into())
        .await?;
    Ok(web::Json(refresh_token.into()))
}

pub async fn update_refresh_token_handler(
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    post_data: web::Json<UpdateRefreshTokenDTO>,
) -> Result<web::Json<RefreshTokenDTO>, ApiError> {
    let refresh_token = refresh_token_service
        .update(post_data.into_inner().into())
        .await?;
    Ok(web::Json(refresh_token.into()))
}

pub async fn list_refresh_tokens_handler(
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    params: web::Query<RefreshTokenQueryParams>,
) -> Result<web::Json<ResultPaging<RefreshTokenDTO>>, ApiError> {
    let selection = refresh_token_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_refresh_token_handler(
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    params: String,
) -> Result<web::Json<RefreshTokenDTO>, ApiError> {
    let refresh_token = refresh_token_service.get(Uuid::from_str(&params)?).await?;
    Ok(web::Json(refresh_token.into()))
}

pub async fn delete_refresh_token_handler(
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    params: String,
) -> Result<HttpResponse, ApiError> {
    refresh_token_service
        .delete(Uuid::from_str(&params)?)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
