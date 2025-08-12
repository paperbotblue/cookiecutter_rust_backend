use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::role_permission::RolePermissionQueryParams;
use crate::domain::repositories::role_permission::RolePermissionRepository;
use crate::domain::services::role_permission::RolePermissionService;

#[derive(Clone)]
pub struct RolePermissionServiceImpl {
    pub repository: Arc<dyn RolePermissionRepository>,
}

impl RolePermissionServiceImpl {
    pub fn new(repository: Arc<dyn RolePermissionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl RolePermissionService for RolePermissionServiceImpl {
    async fn create(&self, item: CreateRolePermission) -> Result<RolePermission, CommonError> {
        self.repository.create(&item).await.map_err(|e| e.into())
    }

    async fn list(
        &self,
        params: RolePermissionQueryParams,
    ) -> Result<ResultPaging<RolePermission>, CommonError> {
        self.repository.list(params).await.map_err(|e| e.into())
    }

    async fn get(&self, item_id1: Uuid, item_id2: Uuid) -> Result<RolePermission, CommonError> {
        self.repository
            .get(item_id1, item_id2)
            .await
            .map_err(|e| e.into())
    }

    async fn delete(&self, item_id1: Uuid, item_id2: Uuid) -> Result<(), CommonError> {
        self.repository
            .delete(item_id1, item_id2)
            .await
            .map_err(|e| e.into())
    }
}
