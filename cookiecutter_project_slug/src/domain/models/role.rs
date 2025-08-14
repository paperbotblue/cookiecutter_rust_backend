use serde::{Deserialize, Serialize};
use std::fmt;
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ClientType {
    Admin,
    User,
    Invalid,
}

impl From<String> for ClientType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "admin" => ClientType::Admin,
            "user" => ClientType::User,
            _ => ClientType::Invalid,
        }
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ClientType::Admin => "admin",
            ClientType::User => "user",
            ClientType::Invalid => "invalid",
        };
        write!(f, "{}", s)
    }
}
