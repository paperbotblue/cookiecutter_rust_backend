use crate::domain::models::role::ClientType;
use chrono::{DateTime, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::token::validate_token_fields;

#[derive(Serialize)]
pub struct CreateTokenDTO {
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UpdateTokenDTO {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TokenDTO {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct RawTokenDTO {
    pub id: Option<Uuid>,
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'de> Deserialize<'de> for CreateTokenDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawTokenDTO::deserialize(deserializer)?;

        validate_token_fields::<D>(&raw.token, &raw.client_type)?;

        Ok(CreateTokenDTO {
            client_id: raw.client_id,
            client_type: raw.client_type,
            is_revoked: raw.is_revoked,
            token: raw.token,
            expires_at: raw.expires_at,
            created_at: raw.created_at,
            updated_at: raw.updated_at,
        })
    }
}

impl<'de> Deserialize<'de> for UpdateTokenDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawTokenDTO::deserialize(deserializer)?;

        let id = raw
            .id
            .ok_or_else(|| D::Error::custom("ID is required for update"))?;

        validate_token_fields::<D>(&raw.token, &raw.client_type)?;

        Ok(UpdateTokenDTO {
            id,
            client_id: raw.client_id,
            client_type: raw.client_type,
            is_revoked: raw.is_revoked,
            token: raw.token,
            expires_at: raw.expires_at,
            created_at: raw.created_at,
            updated_at: raw.updated_at,
        })
    }
}
