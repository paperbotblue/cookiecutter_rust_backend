use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateRolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateRolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}
