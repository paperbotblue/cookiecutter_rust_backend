use std::str::FromStr;

use actix_web::{web, HttpRequest, HttpResponse, Result};
use rand::Rng;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::api::dto::refresh_token::JwtClaims;
use crate::api::dto::user::{CreateUserDTO, LoginDTO, UpdatePasswordDTO, UpdateUserDTO, UserDTO};
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::services::refresh_token::RefreshTokenService;
use crate::domain::services::user::UserService;
use base::error::{ApiError, ApiResponse};
use base::result_paging::ResultPaging;
use shared::mail_handler::send_mail;

// TODO: user should be using role_id insted of role
pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;
    Ok(ApiResponse(user.into()))
}

pub async fn login_user_handler(
    user_service: web::Data<dyn UserService>,
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    post_data: web::Json<LoginDTO>,
) -> Result<HttpResponse, ApiError> {
    let user = user_service.login(post_data.into_inner()).await?;
    let raw_token = refresh_token_service.create_raw_token();
    refresh_token_service
        .create_new_user_refresh_token(user.id, raw_token)
        .await?;
    let jwt_token = refresh_token_service
        .create_user_jwt_token(user.id, user.role.clone())
        .await?;

    let cookie = refresh_token_service.build_refresh_token_cookie(raw_token.to_string())?;

    let res = serde_json::json!({
        "access_token": jwt_token,
        "user_data": UserDTO::from(user),
    });

    Ok(HttpResponse::Ok().cookie(cookie).json(json!({
        "data": res,
        "msg": "",
        "success": true,
        "statusCode": 200
    })))
}

pub async fn logout_user_handler(
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    req: HttpRequest,
) -> Result<ApiResponse<()>, ApiError> {
    let old_raw_token = refresh_token_service.extract_refresh_token(&req)?;
    let old_refresh_token = refresh_token_service
        .get_refresh_token_from_raw_token(old_raw_token.clone())
        .await?;

    refresh_token_service
        .revoke_refresh_token(old_refresh_token.family_id)
        .await?;
    Ok(ApiResponse(()))
}

pub async fn update_user_password_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<UpdatePasswordDTO>,
    claims: JwtClaims,
) -> Result<ApiResponse<Value>, ApiError> {
    user_service
        .update_password(post_data.into_inner().into(), claims.sub)
        .await?;
    Ok(ApiResponse(json!({})))
}

// TODO: user can be sending otp to mobile or email with data somthing like this
// body: {
//   "success": true,
//   "msg": "OTP sent successfully",
//   "statusCode": 200,
//   "data": {
//     "delivery": "email",
//   }
// }

pub async fn send_otp_handler(
    user_service: web::Data<dyn UserService>,
    claims: JwtClaims,
) -> Result<ApiResponse<Value>, ApiError> {
    let mut rng = rand::thread_rng();
    let num: u16 = rng.gen_range(0..10000); // 0..9999 inclusive
    let otp = format!("{:04}", num);

    let user = user_service.get(claims.sub).await?;

    user_service.send_otp(claims.sub, otp.clone()).await?;
    let _ = send_mail(
        &user.email,
        "OTP VERIFICATION",
        &format!("your otp is: {}", otp),
    );

    Ok(ApiResponse(json!({})))
}

#[allow(clippy::too_many_arguments)]
pub async fn get_user_profile_handler(
    user_service: web::Data<dyn UserService>,
    claims: JwtClaims,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.get(claims.sub).await?;
    Ok(ApiResponse(user.into()))
}

pub async fn update_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<UpdateUserDTO>,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.update(post_data.into_inner().into()).await?;
    Ok(ApiResponse(user.into()))
}

pub async fn list_users_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Query<UserQueryParams>,
) -> Result<ApiResponse<ResultPaging<UserDTO>>, ApiError> {
    let selection = user_service.list(params.into_inner()).await?;
    Ok(ApiResponse(selection.map(|item| item.into())))
}

pub async fn get_user_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Path<String>,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.get(Uuid::from_str(&params)?).await?;
    Ok(ApiResponse(user.into()))
}

pub async fn delete_user_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    user_service.delete(Uuid::from_str(&params)?).await?;
    Ok(HttpResponse::NoContent().finish())
}
