use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::repository::ResultPaging;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateRolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateRolePermissionDTO {
    pub id: Uuid,
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct RolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}
