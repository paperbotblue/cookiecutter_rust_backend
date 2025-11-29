use crate::{
    api::dto::refresh_token::{CreateRefreshTokenDTO, RefreshTokenDTO, UpdateRefreshTokenDTO},
    domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken},
};

impl From<RefreshToken> for RefreshTokenDTO {
    fn from(value: RefreshToken) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            token: value.token,
            family_id: value.family_id,
            issued_at: value.issued_at,
            expires_at: value.expires_at,
            is_revoked: value.is_revoked,
        }
    }
}

impl From<CreateRefreshTokenDTO> for CreateRefreshToken {
    fn from(value: CreateRefreshTokenDTO) -> Self {
        Self {
            user_id: value.user_id,
            token: value.token,
            family_id: value.family_id,
            issued_at: value.issued_at,
            expires_at: value.expires_at,
            is_revoked: value.is_revoked,
        }
    }
}

impl From<UpdateRefreshTokenDTO> for UpdateRefreshToken {
    fn from(value: UpdateRefreshTokenDTO) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            token: value.token,
            family_id: value.family_id,
            issued_at: value.issued_at,
            expires_at: value.expires_at,
            is_revoked: value.is_revoked,
        }
    }
}
