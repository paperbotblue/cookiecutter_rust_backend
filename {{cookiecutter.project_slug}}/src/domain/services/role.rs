use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::role_errors::RoleError;
use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::role::RoleQueryParams;

#[async_trait]
pub trait RoleService: 'static + Sync + Send {
    async fn create(&self, role: CreateRole) -> Result<Role, RoleError>;
    async fn update(&self, role: UpdateRole) -> Result<Role, RoleError>;
    async fn list(&self, params: RoleQueryParams) -> Result<ResultPaging<Role>, RoleError>;
    async fn get(&self, role_id: Uuid) -> Result<Role, RoleError>;
    async fn check_permission(&self, role: &str, permision: &str) -> Result<(), RoleError>;
    async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<Permission>, RoleError>;
    async fn delete(&self, role_id: Uuid) -> Result<(), RoleError>;
}
