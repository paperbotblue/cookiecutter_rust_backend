use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::errors::permission_errors::PermissionError;
use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::repositories::permission::PermissionRepository;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::permission::PermissionService;

#[derive(Clone)]
pub struct PermissionServiceImpl {
    pub repository: Arc<dyn PermissionRepository>,
}

impl PermissionServiceImpl {
    pub fn new(repository: Arc<dyn PermissionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl PermissionService for PermissionServiceImpl {
    async fn create(&self, item: CreatePermission) -> Result<Permission, CommonError> {
        if let Some(_) = self
            .repository
            .get_by_name(item.name.clone())
            .await
            .map_err(|e| PermissionError::InternalServerError(e.message))?
        {
            return Err(PermissionError::PermissionAlreadyExists.into());
        }

        self.repository.create(item).await.map_err(|e| e.into())
    }

    async fn update(&self, item: UpdatePermission) -> Result<Permission, CommonError> {
        self.repository.update(item).await.map_err(|e| e.into())
    }

    async fn list(
        &self,
        params: PermissionQueryParams,
    ) -> Result<ResultPaging<Permission>, CommonError> {
        self.repository.list(params).await.map_err(|e| e.into())
    }

    async fn get(&self, item_id: Uuid) -> Result<Permission, CommonError> {
        let permission = self
            .repository
            .get(item_id)
            .await
            .map_err(|e| PermissionError::InternalServerError(e.message))?;
        match permission {
            Some(permission) => Ok(permission),
            None => Err(PermissionError::PermissionDoesNotExist.into()),
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), CommonError> {
        self.repository
            .delete(item_id)
            .await
            .map_err(|e| PermissionError::InternalServerError(e.message.to_string()).into())
    }
}
