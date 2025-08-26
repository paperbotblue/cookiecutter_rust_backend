use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for UserQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User>;
    async fn update(&self, update_user: &UpdateUser) -> RepositoryResult<User>;
    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>>;
    async fn get(&self, user_id: Uuid) -> RepositoryResult<Option<User>>;
    async fn get_by_email(&self, item_id: String) -> RepositoryResult<Option<User>>;
    async fn get_by_phone_number(&self, item_id: String) -> RepositoryResult<Option<User>>;
    async fn get_by_username(&self, item_id: String) -> RepositoryResult<Option<User>>;
    async fn delete(&self, user_id: Uuid) -> RepositoryResult<()>;
}
