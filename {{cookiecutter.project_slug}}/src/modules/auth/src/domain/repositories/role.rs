use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use async_trait::async_trait;
use base::result_paging::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for RoleQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, new_role: &CreateRole) -> RepositoryResult<Role>;
    async fn update(&self, update_role: &UpdateRole) -> RepositoryResult<Role>;
    async fn list(&self, params: RoleQueryParams) -> RepositoryResult<ResultPaging<Role>>;
    async fn get(&self, role_id: Uuid) -> RepositoryResult<Option<Role>>;
    async fn get_by_role_name(&self, role_name: String) -> RepositoryResult<Option<Role>>;
    async fn get_all_permissions_by_role_id(
        &self,
        role_id_val: Uuid,
    ) -> RepositoryResult<Vec<Permission>>;
    async fn is_role_authorized(
        &self,
        role_name: String,
        permission_name: String,
    ) -> RepositoryResult<bool>;
    async fn delete(&self, role_id: Uuid) -> RepositoryResult<()>;
}
