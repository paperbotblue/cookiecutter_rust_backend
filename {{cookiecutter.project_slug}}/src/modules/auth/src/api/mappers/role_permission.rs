use crate::{
    api::dto::role_permission::{CreateRolePermissionDTO, RolePermissionDTO},
    domain::models::role_permission::{CreateRolePermission, RolePermission},
};
impl From<RolePermission> for RolePermissionDTO {
    fn from(val: RolePermission) -> Self {
        RolePermissionDTO {
            role_id: val.role_id,
            permission_id: val.permission_id,
            description: val.description,
        }
    }
}

impl From<CreateRolePermissionDTO> for CreateRolePermission {
    fn from(val: CreateRolePermissionDTO) -> Self {
        CreateRolePermission {
            role_id: val.role_id,
            permission_id: val.permission_id,
            description: val.description,
        }
    }
}

impl From<CreateRolePermission> for CreateRolePermissionDTO {
    fn from(val: CreateRolePermission) -> Self {
        CreateRolePermissionDTO {
            role_id: val.role_id,
            permission_id: val.permission_id,
            description: val.description,
        }
    }
}
