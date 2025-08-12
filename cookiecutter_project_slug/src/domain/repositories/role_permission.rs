use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for RolePermissionQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait RolePermissionRepository: Send + Sync {
    async fn create(
        &self,
        new_role_permission: &CreateRolePermission,
    ) -> RepositoryResult<RolePermission>;
    async fn list(
        &self,
        params: RolePermissionQueryParams,
    ) -> RepositoryResult<ResultPaging<RolePermission>>;
    async fn get(&self, item_id1: Uuid, item_id2: Uuid) -> RepositoryResult<RolePermission>;
    async fn delete(&self, item_id1: Uuid, item_id2: Uuid) -> RepositoryResult<()>;
}
