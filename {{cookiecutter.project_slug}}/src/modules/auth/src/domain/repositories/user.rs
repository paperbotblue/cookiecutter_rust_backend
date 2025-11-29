use crate::domain::models::user::{CreateUser, UpdateUser, User};
use async_trait::async_trait;
use base::result_paging::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use chrono::{DateTime, Utc};
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
    async fn update_password(&self, new_password: String, user_id: Uuid) -> RepositoryResult<User>;
    async fn set_new_otp(
        &self,
        user_id: Uuid,
        otp: String,
        otp_expiry: DateTime<Utc>,
    ) -> RepositoryResult<User>;
    async fn increment_otp_attempts(&self, user_id: Uuid) -> RepositoryResult<User>;
    async fn reset_otp_attempts(&self, user_id: Uuid) -> RepositoryResult<User>;
    async fn expire_otp(&self, user_id: Uuid) -> RepositoryResult<User>;
    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>>;
    async fn get(&self, user_id: Uuid) -> RepositoryResult<Option<User>>;
    async fn get_by_email(&self, user_email: String) -> RepositoryResult<Option<User>>;
    async fn delete(&self, user_id: Uuid) -> RepositoryResult<()>;
}
