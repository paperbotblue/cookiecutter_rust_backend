use crate::{
    api::dto::role::{CreateRoleDTO, RoleDTO, UpdateRoleDTO},
    domain::models::role::{CreateRole, Role, UpdateRole},
};

impl From<Role> for RoleDTO {
    fn from(val: Role) -> Self {
        RoleDTO {
            id: val.id,
            name: val.name,
            description: val.description,
        }
    }
}

impl From<CreateRoleDTO> for CreateRole {
    fn from(val: CreateRoleDTO) -> Self {
        CreateRole {
            name: val.name,
            description: val.description,
        }
    }
}

impl From<UpdateRoleDTO> for UpdateRole {
    fn from(val: UpdateRoleDTO) -> Self {
        UpdateRole {
            id: val.id,
            name: val.name,
            description: val.description,
        }
    }
}
