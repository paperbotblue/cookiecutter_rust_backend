use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::infrastructure::schema::refresh_tokens;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct RefreshTokenDiesel {
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

impl From<RefreshToken> for RefreshTokenDiesel {
    fn from(t: RefreshToken) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            token: t.token,
            family_id: t.family_id,
            issued_at: t.issued_at,
            expires_at: t.expires_at,
            is_revoked: t.is_revoked,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct CreateRefreshTokenDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = refresh_tokens)]
pub struct UpdateRefreshTokenDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

impl From<RefreshTokenDiesel> for RefreshToken {
    fn from(d: RefreshTokenDiesel) -> Self {
        Self {
            id: d.id,
            user_id: d.user_id,
            token: d.token,
            family_id: d.family_id,
            issued_at: d.issued_at,
            expires_at: d.expires_at,
            is_revoked: d.is_revoked,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

impl From<CreateRefreshToken> for CreateRefreshTokenDiesel {
    fn from(t: CreateRefreshToken) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: t.user_id,
            token: t.token,
            family_id: t.family_id,
            issued_at: t.issued_at,
            expires_at: t.expires_at,
            is_revoked: t.is_revoked,
        }
    }
}

impl From<&CreateRefreshToken> for CreateRefreshTokenDiesel {
    fn from(t: &CreateRefreshToken) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: t.user_id,
            token: t.token.clone(),
            family_id: t.family_id,
            issued_at: t.issued_at,
            expires_at: t.expires_at,
            is_revoked: t.is_revoked,
        }
    }
}

impl From<UpdateRefreshToken> for UpdateRefreshTokenDiesel {
    fn from(t: UpdateRefreshToken) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            token: t.token,
            family_id: t.family_id,
            issued_at: t.issued_at,
            expires_at: t.expires_at,
            is_revoked: t.is_revoked,
        }
    }
}

impl From<&UpdateRefreshToken> for UpdateRefreshTokenDiesel {
    fn from(t: &UpdateRefreshToken) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            token: t.token.clone(),
            family_id: t.family_id,
            issued_at: t.issued_at,
            expires_at: t.expires_at,
            is_revoked: t.is_revoked,
        }
    }
}

impl From<CreateRefreshTokenDiesel> for RefreshToken {
    fn from(d: CreateRefreshTokenDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: d.user_id,
            token: d.token,
            family_id: d.family_id,
            issued_at: d.issued_at,
            expires_at: d.expires_at,
            is_revoked: d.is_revoked,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
