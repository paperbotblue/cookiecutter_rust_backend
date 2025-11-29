use actix_web::cookie::Cookie;
use actix_web::dev::ServiceRequest;
use actix_web::HttpRequest;
use async_trait::async_trait;
use uuid::Uuid;

use crate::api::dto::refresh_token::JwtClaims;
use crate::domain::errors::refresh_token_errors::RefreshTokenError;
use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;
use base::result_paging::ResultPaging;

#[async_trait]
pub trait RefreshTokenService: 'static + Sync + Send {
    async fn create_new_user_refresh_token(
        &self,
        user_id: Uuid,
        raw_token: Uuid,
    ) -> Result<RefreshToken, RefreshTokenError>;
    async fn revoke_refresh_token(&self, id: Uuid) -> Result<(), RefreshTokenError>;
    async fn create_user_jwt_token(
        &self,
        user_id: Uuid,
        role: String,
    ) -> Result<String, RefreshTokenError>;

    fn build_refresh_token_cookie(
        &self,
        raw_token: String,
    ) -> Result<Cookie<'_>, RefreshTokenError>;
    async fn renew_refresh_token(&self, raw_token: String) -> Result<String, RefreshTokenError>;
    async fn create(
        &self,
        refresh_token: CreateRefreshToken,
    ) -> Result<RefreshToken, RefreshTokenError>;
    async fn update(
        &self,
        refresh_token: UpdateRefreshToken,
    ) -> Result<RefreshToken, RefreshTokenError>;
    async fn list(
        &self,
        params: RefreshTokenQueryParams,
    ) -> Result<ResultPaging<RefreshToken>, RefreshTokenError>;
    async fn get(&self, refresh_token_id: Uuid) -> Result<RefreshToken, RefreshTokenError>;
    async fn get_refresh_token_from_raw_token(
        &self,
        item_id: String,
    ) -> Result<RefreshToken, RefreshTokenError>;
    async fn delete(&self, refresh_token_id: Uuid) -> Result<(), RefreshTokenError>;
    fn create_raw_token(&self) -> Uuid;
    fn hash_token(&self, token: &str) -> String;
    fn verify_jwt(&self, token: &str) -> Result<JwtClaims, RefreshTokenError>;
    fn extract_refresh_token(&self, req: &HttpRequest) -> Result<String, RefreshTokenError>;
    fn extract_token(&self, req: &ServiceRequest) -> Result<String, RefreshTokenError>;
}
