use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::todo::validate_todo_fields;

#[derive(Serialize)]
pub struct CreateTodoDTO {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct UpdateTodoDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct TodoDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawTodoDTO {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
}

impl<'de> Deserialize<'de> for CreateTodoDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawTodoDTO::deserialize(deserializer)?;

        validate_todo_fields::<D>()?;

        Ok(CreateTodoDTO {
            name: raw.name,
            description: raw.description,
        })
    }
}

impl<'de> Deserialize<'de> for UpdateTodoDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawTodoDTO::deserialize(deserializer)?;

        let id = raw
            .id
            .ok_or_else(|| D::Error::custom("ID is required for update"))?;

        validate_todo_fields::<D>()?;

        Ok(UpdateTodoDTO {
            id,
            name: raw.name,
            description: raw.description,
        })
    }
}
