use async_trait::async_trait;
use uuid::Uuid;

use crate::api::dto::user::LoginDTO;
use crate::domain::errors::user_errors::UserError;
use crate::domain::models::user::{CreateUser, UpdatePassword, UpdateUser, User};
use crate::domain::repositories::user::UserQueryParams;
use base::result_paging::ResultPaging;

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn create(&self, mut user: CreateUser) -> Result<User, UserError>;
    async fn login(&self, item: LoginDTO) -> Result<User, UserError>;
    async fn update(&self, user: UpdateUser) -> Result<User, UserError>;
    async fn send_otp(&self, user_id: Uuid, otp: String) -> Result<(), UserError>;
    async fn update_password(
        &self,
        password: UpdatePassword,
        user_id: Uuid,
    ) -> Result<(), UserError>;
    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, UserError>;
    async fn get(&self, user_id: Uuid) -> Result<User, UserError>;
    async fn delete(&self, user_id: Uuid) -> Result<(), UserError>;
    fn hash_password(&self, token: &str) -> String;
    fn verify_password(&self, password: &str, password_hash: &str) -> bool;
}
