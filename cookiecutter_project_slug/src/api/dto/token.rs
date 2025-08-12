use crate::domain::models::role::ClientType;
use crate::domain::models::token::{CreateToken, Token, UpdateToken};
use crate::domain::repositories::repository::ResultPaging;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateTokenDTO {
    pub client_id: Uuid,
    pub client_type: String,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTokenDTO {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: String,
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
