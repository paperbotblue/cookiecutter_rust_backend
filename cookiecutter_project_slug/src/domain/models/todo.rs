use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub struct CreateTodo {
    pub name: String,
    pub description: String,
}

pub struct UpdateTodo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
