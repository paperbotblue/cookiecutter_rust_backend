use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for PermissionQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create(&self, new_permission: &CreatePermission) -> RepositoryResult<Permission>;
    async fn update(&self, update_permission: &UpdatePermission) -> RepositoryResult<Permission>;
    async fn list(
        &self,
        params: PermissionQueryParams,
    ) -> RepositoryResult<ResultPaging<Permission>>;
    async fn get(&self, permission_id: Uuid) -> RepositoryResult<Permission>;
    async fn delete(&self, permission_id: Uuid) -> RepositoryResult<()>;
}
