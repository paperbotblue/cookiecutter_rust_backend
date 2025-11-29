use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::role_errors::RoleError;
use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::role::RoleQueryParams;
use crate::domain::repositories::role::RoleRepository;
use crate::domain::services::role::RoleService;
use crate::infrastructure::repositories::role::RoleDieselRepository;
use base::result_paging::ResultPaging;

#[derive(Clone)]
pub struct RoleServiceImpl {
    pub repository: Arc<dyn RoleRepository>,
}

impl RoleServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: Arc::new(RoleDieselRepository::new(pool)),
        }
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

    async fn check_permission(
        &self,
        role_name: String,
        permission_name: String,
    ) -> Result<bool, RoleError> {
        match self
            .repository
            .is_role_authorized(role_name, permission_name)
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
