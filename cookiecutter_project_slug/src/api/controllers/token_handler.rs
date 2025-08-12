use crate::api::dto::token::{CreateTokenDTO, TokenDTO, UpdateTokenDTO};
use crate::domain::error::{ApiError, CommonError};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::token::TokenQueryParams;
use crate::domain::services::token::TokenService;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn create_refresh_token_handler(
    refresh_token_service: web::Data<dyn TokenService>,
    post_data: web::Json<CreateTokenDTO>,
) -> Result<web::Json<TokenDTO>, ApiError> {
    let refresh_token = refresh_token_service
        .create(post_data.into_inner().into())
        .await?;
    Ok(web::Json(refresh_token.into()))
}

#[derive(Deserialize)]
pub struct TokenPayload {
    pub token: String,
}

pub async fn check_token_validation(
    refresh_token_service: web::Data<dyn TokenService>,
    post_data: web::Json<TokenPayload>,
) -> Result<web::Json<Value>, ApiError> {
    match refresh_token_service
        .verify_jwt_token(&post_data.token)
        .map_err(|e| CommonError::new(e.to_string(), 600))
    {
        Ok(_) => Ok(web::Json(json!({ "is_token_valid": true }))),
        Err(_) => Err(CommonError::new("invalid token", 600).into()),
    }
}

pub async fn update_refresh_token_handler(
    refresh_token_service: web::Data<dyn TokenService>,
    post_data: web::Json<UpdateTokenDTO>,
) -> Result<web::Json<TokenDTO>, ApiError> {
    let refresh_token = refresh_token_service
        .update(post_data.into_inner().into())
        .await?;
    Ok(web::Json(refresh_token.into()))
}

pub async fn list_refresh_tokens_handler(
    refresh_token_service: web::Data<dyn TokenService>,
    params: web::Query<TokenQueryParams>,
) -> Result<web::Json<ResultPaging<TokenDTO>>, ApiError> {
    let selection = refresh_token_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_refresh_token_handler(
    refresh_token_service: web::Data<dyn TokenService>,
    params: web::Path<Uuid>,
) -> Result<web::Json<TokenDTO>, ApiError> {
    let refresh_token = refresh_token_service.get(params.into_inner()).await?;
    Ok(web::Json(refresh_token.into()))
}

pub async fn delete_refresh_token_handler(
    refresh_token_service: web::Data<dyn TokenService>,
    params: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    refresh_token_service.delete(params.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
