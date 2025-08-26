use uuid::Uuid;

pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}

pub struct CreateRolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub description: String,
}
