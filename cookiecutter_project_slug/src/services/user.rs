use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::user_errors::UserError;
use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, item: CreateUser) -> Result<User, UserError> {
        match self.repository.get_by_email(item.email.clone()).await {
            Ok(Some(_)) => return Err(UserError::UserAlreadyExistsEmail),
            Ok(None) => {}
            Err(err) => return Err(UserError::InternalServerError(err)),
        };

        if let Some(phone_number) = item.phone_number.clone() {
            match self.repository.get_by_phone_number(phone_number).await {
                Ok(Some(_)) => return Err(UserError::UserAlreadyExistsPhoneNumber),
                Ok(None) => {}
                Err(err) => return Err(UserError::InternalServerError(err)),
            }
        }

        if let Some(username) = item.username.clone() {
            match self.repository.get_by_username(username).await {
                Ok(Some(_)) => return Err(UserError::UserAlreadyExistsPhoneNumber),
                Ok(None) => {}
                Err(err) => return Err(UserError::InternalServerError(err)),
            }
        }

        match self.repository.create(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn update(&self, item: UpdateUser) -> Result<User, UserError> {
        match self.repository.update(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, UserError> {
        match self.repository.list(params).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn get(&self, item_id: Uuid) -> Result<User, UserError> {
        match self.repository.get(item_id).await {
            Ok(Some(item)) => Ok(item),
            Ok(None) => Err(UserError::UserDoesNotExist),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), UserError> {
        match self.repository.delete(item_id).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }
}
