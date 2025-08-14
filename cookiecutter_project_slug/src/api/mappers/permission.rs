use crate::{
    api::dto::permission::{CreatePermissionDTO, PermissionDTO, UpdatePermissionDTO},
    domain::{
        models::permission::{CreatePermission, Permission, UpdatePermission},
        repositories::repository::ResultPaging,
    },
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

impl From<ResultPaging<Permission>> for ResultPaging<PermissionDTO> {
    fn from(val: ResultPaging<Permission>) -> Self {
        ResultPaging {
            total: val.total,
            items: val.items.into_iter().map(|item| item.into()).collect(),
        }
    }
}
