use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::CommonError;
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
    async fn create(&self, item: CreateRole) -> Result<Role, CommonError> {
        if let Some(_) = self
            .repository
            .get_by_role_name(item.name.clone())
            .await
            .map_err(|e| RoleError::InternalServerError(e.message))?
        {
            return Err(RoleError::RoleAlreadyExist.into());
        }

        self.repository.create(&item).await.map_err(|e| e.into())
    }

    async fn update(&self, item: UpdateRole) -> Result<Role, CommonError> {
        self.repository.update(&item).await.map_err(|e| e.into())
    }

    async fn list(&self, params: RoleQueryParams) -> Result<ResultPaging<Role>, CommonError> {
        self.repository.list(params).await.map_err(|e| e.into())
    }

    async fn get(&self, item_id: Uuid) -> Result<Role, CommonError> {
        let result = self
            .repository
            .get(item_id)
            .await
            .map_err(|e| RoleError::InternalServerError(e.message.clone()))?;
        match result {
            Some(role) => Ok(role),
            None => Err(RoleError::RoleDoesNotExists.into()),
        }
    }

    async fn check_permission(&self, role: &str, permission: &str) -> Result<(), CommonError> {
        let res = self
            .repository
            .has_permission(role, permission)
            .await
            .map_err(|e| CommonError {
                message: e.message,
                code: 500,
            })?;
        if !res {
            return Err(RoleError::RoleNotAuthorised.into());
        }
        Ok(())
    }

    async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<Permission>, CommonError> {
        self.repository
            .get_all_permissions_by_role_id(role_id)
            .await
            .map_err(|e| CommonError {
                message: e.message,
                code: 500,
            })
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), CommonError> {
        self.repository.delete(item_id).await.map_err(|e| e.into())
    }
}
