use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::infrastructure::schema::roles;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct RoleDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<Role> for RoleDiesel {
    fn from(t: Role) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct CreateRoleDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = roles)]
pub struct UpdateRoleDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<RoleDiesel> for Role {
    fn from(d: RoleDiesel) -> Self {
        Self {
            id: d.id,
            name: d.name,
            description: d.description,
        }
    }
}

impl From<CreateRole> for CreateRoleDiesel {
    fn from(t: CreateRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: t.name,
            description: t.description,
        }
    }
}

impl From<&CreateRole> for CreateRoleDiesel {
    fn from(t: &CreateRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: t.name.clone(),
            description: t.description.clone(),
        }
    }
}

impl From<UpdateRole> for UpdateRoleDiesel {
    fn from(t: UpdateRole) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
        }
    }
}

impl From<&UpdateRole> for UpdateRoleDiesel {
    fn from(t: &UpdateRole) -> Self {
        Self {
            id: t.id,
            name: t.name.clone(),
            description: t.description.clone(),
        }
    }
}

impl From<CreateRoleDiesel> for Role {
    fn from(d: CreateRoleDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: d.name,
            description: d.description,
        }
    }
}
