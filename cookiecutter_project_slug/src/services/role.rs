use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::role_errors::RoleError;
use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::role::RoleQueryParams;
use crate::domain::repositories::role::RoleRepository;
use crate::domain::services::role::RoleService;

#[derive(Clone)]
pub struct RoleServiceImpl {
    pub repository: Arc<dyn RoleRepository>,
}

impl RoleServiceImpl {
    pub fn new(repository: Arc<dyn RoleRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl RoleService for RoleServiceImpl {
    async fn create(&self, item: CreateRole) -> Result<Role, RoleError> {
        match self.repository.get_by_role_name(item.name.clone()).await {
            Ok(Some(_)) => return Err(RoleError::RoleAlreadyExist),
            Ok(None) => {}
            Err(err) => return Err(RoleError::InternalServerError(err)),
        };

        match self.repository.create(&item).await {
            Ok(role) => Ok(role),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }

    async fn update(&self, item: UpdateRole) -> Result<Role, RoleError> {
        match self.repository.update(&item).await {
            Ok(result) => Ok(result),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }

    async fn list(&self, params: RoleQueryParams) -> Result<ResultPaging<Role>, RoleError> {
        match self.repository.list(params).await {
            Ok(result) => Ok(result),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }

    async fn get(&self, item_id: Uuid) -> Result<Role, RoleError> {
        match self.repository.get(item_id).await {
            Ok(Some(role)) => Ok(role),
            Ok(None) => Err(RoleError::RoleDoesNotExists),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }

    async fn check_permission(&self, role: &str, permission: &str) -> Result<(), RoleError> {
        match self.repository.has_permission(role, permission).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(RoleError::RoleNotAuthorised),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }

    async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<Permission>, RoleError> {
        match self
            .repository
            .get_all_permissions_by_role_id(role_id)
            .await
        {
            Ok(result) => Ok(result),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), RoleError> {
        match self.repository.delete(item_id).await {
            Ok(()) => Ok(()),
            Err(err) => Err(RoleError::InternalServerError(err)),
        }
    }
}
