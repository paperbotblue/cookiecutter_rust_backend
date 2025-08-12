use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::token::{CreateToken, JwtClaims, Token, UpdateToken};
use crate::domain::repositories::repository::{QueryParams, ResultPaging};
use crate::domain::repositories::token::TokenQueryParams;

#[async_trait]
pub trait TokenService: 'static + Sync + Send {
    async fn create(&self, refresh_token: CreateToken) -> Result<Token, CommonError>;
    async fn update(&self, refresh_token: UpdateToken) -> Result<Token, CommonError>;
    async fn list(&self, params: TokenQueryParams) -> Result<ResultPaging<Token>, CommonError>;
    async fn get(&self, refresh_token_id: Uuid) -> Result<Token, CommonError>;
    fn generate_jwt_token(
        &self,
        subject: Uuid,
        permissions: Vec<String>,
    ) -> Result<String, CommonError>;
    fn verify_jwt_token(&self, token: &str) -> Result<JwtClaims, CommonError>;
    fn expiration_check(&self, token: &JwtClaims) -> Result<(), CommonError>;
    async fn generate_refresh_token(
        &self,
        client_id: Uuid,
        client_type: String,
    ) -> Result<String, CommonError>;
    async fn delete(&self, refresh_token_id: Uuid) -> Result<(), CommonError>;
}
