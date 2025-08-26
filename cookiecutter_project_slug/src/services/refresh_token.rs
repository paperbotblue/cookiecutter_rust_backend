use async_trait::async_trait;
use jsonwebtoken::{encode, EncodingKey, Header};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::refresh_token_errors::RefreshTokenError;
use crate::domain::models::refresh_token::{
    CreateRefreshToken, JwtClaims, RefreshToken, UpdateRefreshToken,
};
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::refresh_token::RefreshTokenService;

#[derive(Clone)]
pub struct RefreshTokenServiceImpl {
    pub repository: Arc<dyn RefreshTokenRepository>,
}

impl RefreshTokenServiceImpl {
    pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl RefreshTokenService for RefreshTokenServiceImpl {
    async fn create(&self, item: CreateRefreshToken) -> Result<RefreshToken, RefreshTokenError> {
        match self.repository.create(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    async fn create_new_user_refresh_token(
        &self,
        user_id: Uuid,
        raw_token: Uuid,
    ) -> Result<RefreshToken, RefreshTokenError> {
        // TODO: Get this from .env
        let secret_key = "my_super_secret_key";
        let token_with_key = format!("{}:{}", raw_token, secret_key);
        let secure_token_hash = self.hash_token(&token_with_key);

        let new_user_token = CreateRefreshToken::new(user_id, secure_token_hash);

        match self.repository.create(&new_user_token).await {
            Ok(refresh_token) => Ok(refresh_token),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    async fn create_user_jwt_token(
        &self,
        user_id: Uuid,
        role: String,
    ) -> Result<String, RefreshTokenError> {
        // TODO: Get this from .env
        let secret = "my_secret_key";

        let jwt_token = JwtClaims::new(user_id, role);
        encode(
            &Header::default(),
            &jwt_token,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|err| RefreshTokenError::InternalServerError(err.into()))
    }

    async fn update(&self, item: UpdateRefreshToken) -> Result<RefreshToken, RefreshTokenError> {
        match self.repository.update(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    async fn list(
        &self,
        params: RefreshTokenQueryParams,
    ) -> Result<ResultPaging<RefreshToken>, RefreshTokenError> {
        match self.repository.list(params).await {
            Ok(item) => Ok(item),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    async fn get(&self, item_id: Uuid) -> Result<RefreshToken, RefreshTokenError> {
        match self.repository.get(item_id).await {
            Ok(Some(item)) => Ok(item),
            Ok(None) => Err(RefreshTokenError::RefreshTokenDoesNotExist),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), RefreshTokenError> {
        match self.repository.delete(item_id).await {
            Ok(item) => Ok(item),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }
}
