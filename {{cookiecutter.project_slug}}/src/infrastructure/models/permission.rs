use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::infrastructure::schema::permissions;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct PermissionDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<Permission> for PermissionDiesel {
    fn from(t: Permission) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = permissions)]
pub struct CreatePermissionDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = permissions)]
pub struct UpdatePermissionDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<PermissionDiesel> for Permission {
    fn from(d: PermissionDiesel) -> Self {
        Self {
            id: d.id,
            name: d.name,
            description: d.description,
        }
    }
}

impl From<CreatePermission> for CreatePermissionDiesel {
    fn from(t: CreatePermission) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: t.name,
            description: t.description,
        }
    }
}

impl From<UpdatePermission> for UpdatePermissionDiesel {
    fn from(t: UpdatePermission) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
        }
    }
}

impl From<CreatePermissionDiesel> for Permission {
    fn from(d: CreatePermissionDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: d.name,
            description: d.description,
        }
    }
}
