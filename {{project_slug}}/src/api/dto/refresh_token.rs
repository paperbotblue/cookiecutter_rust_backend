use chrono::{DateTime, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::refresh_token::validate_refresh_token_fields;

#[derive(Serialize)]
pub struct CreateRefreshTokenDTO {
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

#[derive(Serialize)]
pub struct UpdateRefreshTokenDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawRefreshTokenDTO {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'de> Deserialize<'de> for CreateRefreshTokenDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRefreshTokenDTO::deserialize(deserializer)?;

        validate_refresh_token_fields::<D>()?;

        Ok(CreateRefreshTokenDTO {
            user_id: raw.user_id,
            token: raw.token,
            family_id: raw.family_id,
            issued_at: raw.issued_at,
            expires_at: raw.expires_at,
            is_revoked: raw.is_revoked,
        })
    }
}

impl<'de> Deserialize<'de> for UpdateRefreshTokenDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRefreshTokenDTO::deserialize(deserializer)?;

        let id = raw
            .id
            .ok_or_else(|| D::Error::custom("ID is required for update"))?;

        validate_refresh_token_fields::<D>()?;

        Ok(UpdateRefreshTokenDTO {
            id,
            user_id: raw.user_id,
            token: raw.token,
            family_id: raw.family_id,
            issued_at: raw.issued_at,
            expires_at: raw.expires_at,
            is_revoked: raw.is_revoked,
        })
    }
}
