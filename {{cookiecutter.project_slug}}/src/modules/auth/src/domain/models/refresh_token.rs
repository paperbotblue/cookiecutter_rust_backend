use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

pub struct CreateRefreshToken {
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

pub struct UpdateRefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}
