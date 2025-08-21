use async_trait::async_trait;

use chrono::Utc;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::errors::token_errors::TokenError;
use crate::domain::models::token::{CreateToken, JwtClaims, Token, TokenClaims, UpdateToken};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::token::TokenQueryParams;
use crate::domain::repositories::token::TokenRepository;
use crate::domain::services::token::TokenService;

#[derive(Clone)]
pub struct TokenServiceImpl {
    pub repository: Arc<dyn TokenRepository>,
}

impl TokenServiceImpl {
    pub fn new(repository: Arc<dyn TokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl TokenService for TokenServiceImpl {
    async fn create(&self, item: CreateToken) -> Result<Token, CommonError> {
        if self
            .repository
            .get_by_client_id(item.client_id)
            .await
            .map_err(CommonError::from)?
            .is_some()
        {
            return Err(TokenError::RefreshTokenAlreadyExist.into());
        }

        self.repository.create(&item).await.map_err(|e| e.into())
    }

    async fn update(&self, item: UpdateToken) -> Result<Token, CommonError> {
        self.repository.update(&item).await.map_err(|e| e.into())
    }

    async fn list(&self, params: TokenQueryParams) -> Result<ResultPaging<Token>, CommonError> {
        self.repository.list(params).await.map_err(|e| e.into())
    }

    async fn get(&self, item_id: Uuid) -> Result<Token, CommonError> {
        if let Some(token) = self.repository.get(item_id).await.map_err(|e| e)? {
            Ok(token)
        } else {
            return Err(TokenError::RefreshTokenDoesNotExists.into());
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), CommonError> {
        self.repository.delete(item_id).await.map_err(|e| e.into())
    }
    async fn generate_refresh_token(
        &self,
        client_id: Uuid,
        client_type: String,
    ) -> Result<String, CommonError> {
        let now = chrono::Utc::now();
        // TODO: get exp token time from .env
        let exp = now + chrono::Duration::days(30);
        let iat = now;

        let new_refresh_token = CreateToken {
            client_id,
            client_type: client_type.clone().into(),
            is_revoked: false,
            token: "".to_string(),
            expires_at: exp,
            created_at: now,
            updated_at: now,
        };

        let claims = TokenClaims {
            exp: exp.timestamp() as usize,
            iat: iat.timestamp() as usize,

            sub: String::from(client_id),
            sub_type: client_type,
        };

        // TODO: Check if this exists
        self.repository
            .create(&new_refresh_token)
            .await
            .map_err(CommonError::from)?;

        // TODO: get refresh token secrets from .env
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("ksdfjslj23rln23lk32r".to_string().as_bytes()),
        )
        .map_err(CommonError::from)
    }
    fn generate_jwt_token(
        &self,
        subject: Uuid,
        permissions: Vec<String>,
    ) -> Result<String, CommonError> {
        let now = chrono::Utc::now();
        // TODO: get exp token time from .env
        let exp = now + chrono::Duration::minutes(10);
        let iat = now;

        let permissions_string = permissions
            .into_iter() // extract the name
            .collect::<Vec<_>>() // collect names into a Vec<String>
            .join(" "); // join with spaces

        let claims = JwtClaims {
            sub: subject,
            exp: exp.timestamp() as usize,
            iat: iat.timestamp() as usize,
            permissions: permissions_string,
        };

        // TODO: get refresh token secrets from .env
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("ksdfjslj23rln23lk32r".to_string().as_bytes()),
        )
        .map_err(CommonError::from)
    }

    fn verify_jwt_token(&self, token: &str) -> Result<JwtClaims, CommonError> {
        let token_data = jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret("ksdfjslj23rln23lk32r".to_string().as_bytes()),
            &jsonwebtoken::Validation::default(),
        );

        match token_data {
            Ok(t) => Ok(t.claims),
            Err(e) => Err(CommonError::new(e.to_string(), 400)),
        }
    }

    fn expiration_check(&self, token: &JwtClaims) -> Result<(), CommonError> {
        if !token.exp > Utc::now().timestamp() as usize {
            return Err(TokenError::TokenExpired.into());
        }
        Ok(())
    }
}
