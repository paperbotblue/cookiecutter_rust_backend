use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::role::validate_role_fields;

#[derive(Serialize)]
pub struct CreateRoleDTO {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
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

#[derive(Deserialize)]
pub struct RawRoleDTO {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
}

impl<'de> Deserialize<'de> for CreateRoleDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRoleDTO::deserialize(deserializer)?;

        validate_role_fields::<D>(&raw.name, &raw.description)?;

        Ok(CreateRoleDTO {
            name: raw.name,
            description: raw.description,
        })
    }
}

impl<'de> Deserialize<'de> for UpdateRoleDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRoleDTO::deserialize(deserializer)?;

        let id = raw
            .id
            .ok_or_else(|| D::Error::custom("ID is required for update"))?;

        validate_role_fields::<D>(&raw.name, &raw.description)?;

        Ok(UpdateRoleDTO {
            id,
            name: raw.name,
            description: raw.description,
        })
    }
}
