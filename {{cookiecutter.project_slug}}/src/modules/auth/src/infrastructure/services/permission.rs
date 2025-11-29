use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::permission_errors::PermissionError;
use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::repositories::permission::PermissionRepository;
use crate::domain::services::permission::PermissionService;
use crate::infrastructure::repositories::permission::PermissionDbRepository;
use base::result_paging::ResultPaging;

#[derive(Clone)]
pub struct PermissionServiceImpl {
    pub repository: Arc<dyn PermissionRepository>,
}

impl PermissionServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: Arc::new(PermissionDbRepository::new(pool)),
        }
    }
}

#[async_trait]
impl PermissionService for PermissionServiceImpl {
    async fn create(&self, item: CreatePermission) -> Result<Permission, PermissionError> {
        match self.repository.get_by_name(item.name.clone()).await {
            Ok(Some(_)) => return Err(PermissionError::PermissionAlreadyExists),
            Ok(None) => {}
            Err(err) => return Err(PermissionError::InternalServerError(err)),
        };

        match self.repository.create(item).await {
            Ok(permission) => Ok(permission),
            Err(err) => Err(PermissionError::InternalServerError(err)),
        }
    }

    async fn update(&self, item: UpdatePermission) -> Result<Permission, PermissionError> {
        match self.repository.update(item).await {
            Ok(result) => Ok(result),
            Err(err) => Err(PermissionError::InternalServerError(err)),
        }
    }

    async fn list(
        &self,
        params: PermissionQueryParams,
    ) -> Result<ResultPaging<Permission>, PermissionError> {
        match self.repository.list(params).await {
            Ok(result) => Ok(result),
            Err(err) => Err(PermissionError::InternalServerError(err)),
        }
    }

    async fn get(&self, item_id: Uuid) -> Result<Permission, PermissionError> {
        match self.repository.get(item_id).await {
            Ok(Some(permission)) => Ok(permission),
            Ok(None) => Err(PermissionError::PermissionDoesNotExist),
            Err(err) => Err(PermissionError::InternalServerError(err)),
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), PermissionError> {
        match self.repository.delete(item_id).await {
            Ok(result) => Ok(result),
            Err(err) => Err(PermissionError::InternalServerError(err)),
        }
    }
}
