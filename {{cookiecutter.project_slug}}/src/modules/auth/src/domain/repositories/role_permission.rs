use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use async_trait::async_trait;
use base::result_paging::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
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
    async fn get(
        &self,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> RepositoryResult<Option<RolePermission>>;
    async fn delete(&self, role_id: Uuid, permission_id: Uuid) -> RepositoryResult<()>;
}
