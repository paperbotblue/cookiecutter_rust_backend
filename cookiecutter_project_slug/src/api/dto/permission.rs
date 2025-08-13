use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::permission::validate_permission_fields;

#[derive(Serialize)]
pub struct CreatePermissionDTO {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
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

#[derive(Deserialize)]
struct RawPermissionDTO {
    id: Option<Uuid>,
    name: String,
    description: String,
}

impl<'de> Deserialize<'de> for CreatePermissionDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawPermissionDTO::deserialize(deserializer)?;

        validate_permission_fields::<D>(&raw.name, &raw.description)?;

        Ok(CreatePermissionDTO {
            name: raw.name,
            description: raw.description,
        })
    }
}

impl<'de> Deserialize<'de> for UpdatePermissionDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawPermissionDTO::deserialize(deserializer)?;

        let id = raw
            .id
            .ok_or_else(|| D::Error::custom("ID is required for update"))?;

        validate_permission_fields::<D>(&raw.name, &raw.description)?;

        Ok(UpdatePermissionDTO {
            id,
            name: raw.name,
            description: raw.description,
        })
    }
}
