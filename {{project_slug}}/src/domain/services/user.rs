use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::user_errors::UserError;
use crate::domain::models::user::{CreateUser, User, UpdateUser};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn create(&self, user: CreateUser) -> Result<User, UserError>;
    async fn update(&self, user: UpdateUser) -> Result<User, UserError>;
    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, UserError>;
    async fn get(&self, user_id: Uuid) -> Result<User, UserError>;
    async fn delete(&self, user_id: Uuid) -> Result<(), UserError>;
}
