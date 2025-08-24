use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::ApiError;
use crate::domain::errors::role_permission_errors::RolePermissionError;
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
    async fn create(&self, item: CreateRolePermission) -> Result<RolePermission, ApiError> {
        match self.repository.create(&item).await {
            Ok(role_permission) => Ok(role_permission),
            Err(err) => Err(RolePermissionError::InternalServerError(err).into()),
        }
    }

    async fn list(
        &self,
        params: RolePermissionQueryParams,
    ) -> Result<ResultPaging<RolePermission>, ApiError> {
        match self.repository.list(params).await {
            Ok(result) => Ok(result),
            Err(err) => Err(RolePermissionError::InternalServerError(err).into()),
        }
    }

    async fn get(&self, item_id1: Uuid, item_id2: Uuid) -> Result<RolePermission, ApiError> {
        match self.repository.get(item_id1, item_id2).await {
            Ok(result) => Ok(result),
            Err(err) => Err(RolePermissionError::InternalServerError(err).into()),
        }
    }

    async fn delete(&self, item_id1: Uuid, item_id2: Uuid) -> Result<(), ApiError> {
        match self.repository.delete(item_id1, item_id2).await {
            Ok(result) => Ok(result),
            Err(err) => Err(RolePermissionError::InternalServerError(err).into()),
        }
    }
}
