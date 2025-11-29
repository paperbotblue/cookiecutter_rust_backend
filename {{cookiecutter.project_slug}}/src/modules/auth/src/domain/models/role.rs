use uuid::Uuid;

pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub struct CreateRole {
    pub name: String,
    pub description: String,
}

pub struct UpdateRole {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
