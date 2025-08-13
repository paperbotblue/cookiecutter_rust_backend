use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::role_permission::validate_role_permission_fields;

#[derive(Serialize)]
pub struct CreateRolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

#[derive(Serialize)]
pub struct UpdateRolePermissionDTO {
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

#[derive(Deserialize)]
pub struct RawRolePermissionDTO {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

impl<'de> Deserialize<'de> for CreateRolePermissionDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRolePermissionDTO::deserialize(deserializer)?;

        validate_role_permission_fields::<D>(&raw.description)?;

        Ok(CreateRolePermissionDTO {
            role_id: raw.role_id,
            permission_id: raw.permission_id,
            description: raw.description,
        })
    }
}

impl<'de> Deserialize<'de> for UpdateRolePermissionDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRolePermissionDTO::deserialize(deserializer)?;

        validate_role_permission_fields::<D>(&raw.description)?;

        Ok(UpdateRolePermissionDTO {
            role_id: raw.role_id,
            permission_id: raw.permission_id,
            description: raw.description,
        })
    }
}
