use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::ApiError;
use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::role_permission::RolePermissionQueryParams;

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
    async fn get(&self, item_id1: Uuid, item_id2: Uuid) -> Result<RolePermission, ApiError>;
    async fn delete(&self, item_id1: Uuid, item_id2: Uuid) -> Result<(), ApiError>;
}
