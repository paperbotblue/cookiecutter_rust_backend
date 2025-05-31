use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
}

pub struct CreateTodo {
    pub description: String,
    pub is_completed: bool,
}

pub struct UpdateTodo {
    pub id: Uuid,
    pub description: String,
    pub is_completed: bool,
}
