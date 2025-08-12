use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::repository::ResultPaging;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateRoleDTO {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateRoleDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
