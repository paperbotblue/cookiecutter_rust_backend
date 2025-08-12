use crate::{
    api::dto::token::{CreateTokenDTO, TokenDTO, UpdateTokenDTO},
    domain::{
        models::token::{CreateToken, Token, UpdateToken},
        repositories::repository::ResultPaging,
    },
};

impl From<Token> for TokenDTO {
    fn from(val: Token) -> Self {
        TokenDTO {
            id: val.id,
            client_id: val.client_id,
            client_type: val.client_type,
            is_revoked: val.is_revoked,
            token: val.token,
            expires_at: val.expires_at,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl From<CreateTokenDTO> for CreateToken {
    fn from(val: CreateTokenDTO) -> Self {
        CreateToken {
            client_id: val.client_id,
            client_type: val.client_type,
            is_revoked: val.is_revoked,
            token: val.token,
            expires_at: val.expires_at,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl From<UpdateTokenDTO> for UpdateToken {
    fn from(val: UpdateTokenDTO) -> Self {
        UpdateToken {
            id: val.id,
            client_id: val.client_id,
            client_type: val.client_type,
            is_revoked: val.is_revoked,
            token: val.token,
            expires_at: val.expires_at,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl From<CreateToken> for CreateTokenDTO {
    fn from(val: CreateToken) -> Self {
        CreateTokenDTO {
            client_id: val.client_id,
            client_type: val.client_type,
            is_revoked: val.is_revoked,
            token: val.token,
            expires_at: val.expires_at,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl From<UpdateToken> for UpdateTokenDTO {
    fn from(val: UpdateToken) -> Self {
        UpdateTokenDTO {
            id: val.id,
            client_id: val.client_id,
            client_type: val.client_type,
            is_revoked: val.is_revoked,
            token: val.token,
            expires_at: val.expires_at,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl From<ResultPaging<Token>> for ResultPaging<TokenDTO> {
    fn from(val: ResultPaging<Token>) -> Self {
        ResultPaging {
            total: val.total,
            items: val.items.into_iter().map(|item| item.into()).collect(),
        }
    }
}
