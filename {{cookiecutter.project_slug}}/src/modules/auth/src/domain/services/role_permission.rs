use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::role_permission::RolePermissionQueryParams;
use base::error::ApiError;
use base::result_paging::ResultPaging;

#[async_trait]
pub trait RolePermissionService: 'static + Sync + Send {
    async fn create(
        &self,
        role_permission: CreateRolePermission,
    ) -> Result<RolePermission, ApiError>;
    async fn list(
        &self,
        params: RolePermissionQueryParams,
    ) -> Result<ResultPaging<RolePermission>, ApiError>;
    async fn get(&self, role_id: Uuid, permission_id: Uuid) -> Result<RolePermission, ApiError>;
    async fn delete(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), ApiError>;
}
