use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::repositories::repository::ResultPaging;

#[async_trait]
pub trait PermissionService: 'static + Sync + Send {
    async fn create(&self, permission: CreatePermission) -> Result<Permission, CommonError>;
    async fn update(&self, permission: UpdatePermission) -> Result<Permission, CommonError>;
    async fn list(
        &self,
        params: PermissionQueryParams,
    ) -> Result<ResultPaging<Permission>, CommonError>;
    async fn get(&self, permission_id: Uuid) -> Result<Permission, CommonError>;
    async fn delete(&self, permission_id: Uuid) -> Result<(), CommonError>;
}
