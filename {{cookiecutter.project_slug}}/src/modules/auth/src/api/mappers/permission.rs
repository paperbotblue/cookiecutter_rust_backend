use crate::{
    api::dto::permission::{CreatePermissionDTO, PermissionDTO, UpdatePermissionDTO},
    domain::models::permission::{CreatePermission, Permission, UpdatePermission},
};

impl From<Permission> for PermissionDTO {
    fn from(val: Permission) -> Self {
        PermissionDTO {
            id: val.id,
            name: val.name,
            description: val.description,
        }
    }
}

impl From<CreatePermissionDTO> for CreatePermission {
    fn from(val: CreatePermissionDTO) -> Self {
        CreatePermission {
            name: val.name,
            description: val.description,
        }
    }
}

impl From<UpdatePermissionDTO> for UpdatePermission {
    fn from(val: UpdatePermissionDTO) -> Self {
        UpdatePermission {
            id: val.id,
            name: val.name,
            description: val.description,
        }
    }
}
