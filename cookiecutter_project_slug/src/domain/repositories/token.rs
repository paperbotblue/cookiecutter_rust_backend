use crate::domain::models::token::{CreateToken, Token, UpdateToken};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for TokenQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn create(&self, new_token: &CreateToken) -> RepositoryResult<Token>;
    async fn update(&self, update_token: &UpdateToken) -> RepositoryResult<Token>;
    async fn list(&self, params: TokenQueryParams) -> RepositoryResult<ResultPaging<Token>>;
    async fn get(&self, token_id: Uuid) -> RepositoryResult<Token>;
    async fn get_by_client_id(&self, client_id: Uuid) -> RepositoryResult<Option<Token>>;
    async fn delete(&self, refresh_token_id: Uuid) -> RepositoryResult<()>;
}
