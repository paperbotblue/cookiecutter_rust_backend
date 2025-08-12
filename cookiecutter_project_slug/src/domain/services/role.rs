use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::role::RoleQueryParams;

#[async_trait]
pub trait RoleService: 'static + Sync + Send {
    async fn create(&self, role: CreateRole) -> Result<Role, CommonError>;
    async fn update(&self, role: UpdateRole) -> Result<Role, CommonError>;
    async fn list(&self, params: RoleQueryParams) -> Result<ResultPaging<Role>, CommonError>;
    async fn get(&self, role_id: Uuid) -> Result<Role, CommonError>;
    async fn check_permission(&self, role: &str, permision: &str) -> Result<(), CommonError>;
    async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<Permission>, CommonError>;
    async fn delete(&self, role_id: Uuid) -> Result<(), CommonError>;
}
