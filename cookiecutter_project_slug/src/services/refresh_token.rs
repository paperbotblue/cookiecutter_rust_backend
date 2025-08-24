use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::refresh_token_errors::RefreshTokenError ;
use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
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
    async fn create(&self, item: CreateRefreshToken) -> Result<RefreshToken, RefreshTokenError > {
        match self.repository
            .create(&item)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(RefreshTokenError::InternalServerError(err)),
            }
    }

    async fn update(&self, item: UpdateRefreshToken) -> Result<RefreshToken, RefreshTokenError > {
        match self.repository
            .update(&item)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(RefreshTokenError::InternalServerError(err)),
            }
    }

    async fn list(&self, params: RefreshTokenQueryParams) -> Result<ResultPaging<RefreshToken>, RefreshTokenError > {
        match self.repository
            .list(params)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(RefreshTokenError::InternalServerError(err)),
            }
    }

    async fn get(&self, item_id: Uuid) -> Result<RefreshToken, RefreshTokenError > {
        match self.repository
            .get(item_id)
            .await {
              Ok(Some(item)) => Ok(item),
              Ok(None) => Err(RefreshTokenError::RefreshTokenDoesNotExist),
              Err(err) => Err(RefreshTokenError::InternalServerError(err)),
            }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), RefreshTokenError > {
        match self.repository
            .delete(item_id)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(RefreshTokenError::InternalServerError(err)),
            }
    }
}
