use crate::domain::models::token::{CreateToken, Token, UpdateToken};
use crate::infrastructure::schema::refresh_tokens;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct TokenDiesel {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: String,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Token> for TokenDiesel {
    fn from(t: Token) -> Self {
        Self {
            id: t.id,
            client_id: t.client_id,
            client_type: t.client_type.to_string(),
            is_revoked: t.is_revoked,
            token: t.token,
            expires_at: t.expires_at,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct CreateTokenDiesel {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: String,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(AsChangeset)]
#[diesel(table_name = refresh_tokens)]
pub struct UpdateTokenDiesel {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: String,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TokenDiesel> for Token {
    fn from(d: TokenDiesel) -> Self {
        Self {
            id: d.id,
            client_id: d.client_id,
            client_type: d.client_type.into(),
            is_revoked: d.is_revoked,
            token: d.token,
            expires_at: d.expires_at,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

impl From<CreateToken> for CreateTokenDiesel {
    fn from(t: CreateToken) -> Self {
        Self {
            id: Uuid::new_v4(),
            client_id: t.client_id,
            client_type: t.client_type.to_string(),
            is_revoked: t.is_revoked,
            token: t.token,
            expires_at: t.expires_at,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl From<&CreateToken> for CreateTokenDiesel {
    fn from(t: &CreateToken) -> Self {
        Self {
            id: Uuid::new_v4(),
            client_id: t.client_id,
            client_type: t.client_type.to_string(),
            is_revoked: t.is_revoked,
            token: t.token.clone(),
            expires_at: t.expires_at,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl From<UpdateToken> for UpdateTokenDiesel {
    fn from(t: UpdateToken) -> Self {
        Self {
            id: t.id,
            client_id: t.client_id,
            client_type: t.client_type.to_string(),
            is_revoked: t.is_revoked,
            token: t.token,
            expires_at: t.expires_at,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl From<&UpdateToken> for UpdateTokenDiesel {
    fn from(t: &UpdateToken) -> Self {
        Self {
            id: t.id,
            client_id: t.client_id,
            client_type: t.client_type.to_string(),
            is_revoked: t.is_revoked,
            token: t.token.clone(),
            expires_at: t.expires_at,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl From<CreateTokenDiesel> for Token {
    fn from(d: CreateTokenDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            client_id: d.client_id,
            client_type: d.client_type.into(),
            is_revoked: d.is_revoked,
            token: d.token,
            expires_at: d.expires_at,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}
