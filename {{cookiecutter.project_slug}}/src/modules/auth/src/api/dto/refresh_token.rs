use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRefreshTokenDTO {
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

#[derive(Serialize, Deserialize)]
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
}
