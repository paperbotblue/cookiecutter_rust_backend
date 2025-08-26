use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::infrastructure::schema::role_permissions;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct RolePermissionDiesel {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

impl From<RolePermission> for RolePermissionDiesel {
    fn from(t: RolePermission) -> Self {
        Self {
            role_id: t.role_id,
            permission_id: t.permission_id,
            description: t.description,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = role_permissions)]
pub struct CreateRolePermissionDiesel {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

impl From<RolePermissionDiesel> for RolePermission {
    fn from(d: RolePermissionDiesel) -> Self {
        Self {
            role_id: d.role_id,
            permission_id: d.permission_id,
            description: d.description,
        }
    }
}

impl From<CreateRolePermission> for CreateRolePermissionDiesel {
    fn from(t: CreateRolePermission) -> Self {
        Self {
            role_id: t.role_id,
            permission_id: t.permission_id,
            description: t.description,
        }
    }
}

impl From<&CreateRolePermission> for CreateRolePermissionDiesel {
    fn from(t: &CreateRolePermission) -> Self {
        Self {
            role_id: t.role_id,
            permission_id: t.permission_id,
            description: t.description.clone(),
        }
    }
}

impl From<CreateRolePermissionDiesel> for RolePermission {
    fn from(d: CreateRolePermissionDiesel) -> Self {
        Self {
            role_id: d.role_id,
            permission_id: d.permission_id,
            description: d.description,
        }
    }
}
