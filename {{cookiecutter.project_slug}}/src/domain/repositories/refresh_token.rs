use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for RefreshTokenQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(
        &self,
        new_refresh_token: &CreateRefreshToken,
    ) -> RepositoryResult<RefreshToken>;
    async fn update(
        &self,
        update_refresh_token: &UpdateRefreshToken,
    ) -> RepositoryResult<RefreshToken>;
    async fn revoke_family_id(&self, id_val: Uuid) -> RepositoryResult<()>;
    async fn revoke_token(&self, id_val: Uuid) -> RepositoryResult<()>;
    async fn list(
        &self,
        params: RefreshTokenQueryParams,
    ) -> RepositoryResult<ResultPaging<RefreshToken>>;
    async fn get(&self, refresh_token_id: Uuid) -> RepositoryResult<Option<RefreshToken>>;
    async fn get_from_hash(&self, item_id: String) -> RepositoryResult<Option<RefreshToken>>;
    async fn delete(&self, refresh_token_id: Uuid) -> RepositoryResult<()>;
}
