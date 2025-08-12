use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::repository::ResultPaging;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreatePermissionDTO {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePermissionDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct PermissionDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
