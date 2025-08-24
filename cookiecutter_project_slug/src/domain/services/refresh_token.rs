use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::refresh_token_errors::RefreshTokenError;
use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;

#[async_trait]
pub trait RefreshTokenService: 'static + Sync + Send {
    async fn create(&self, refresh_token: CreateRefreshToken) -> Result<RefreshToken, RefreshTokenError>;
    async fn update(&self, refresh_token: UpdateRefreshToken) -> Result<RefreshToken, RefreshTokenError>;
    async fn list(&self, params: RefreshTokenQueryParams) -> Result<ResultPaging<RefreshToken>, RefreshTokenError>;
    async fn get(&self, refresh_token_id: Uuid) -> Result<RefreshToken, RefreshTokenError>;
    async fn delete(&self, refresh_token_id: Uuid) -> Result<(), RefreshTokenError>;
}
