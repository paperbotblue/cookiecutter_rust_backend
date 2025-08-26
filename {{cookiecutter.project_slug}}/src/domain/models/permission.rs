use uuid::Uuid;
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub struct CreatePermission {
    pub name: String,
    pub description: String,
}

pub struct UpdatePermission {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
