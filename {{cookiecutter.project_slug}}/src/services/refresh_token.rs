use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::ServiceRequest;
use actix_web::http::header::AUTHORIZATION;
use actix_web::HttpRequest;
use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::constants;
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
        let secret_key = (constants::REFRESH_TOKEN_SECRET).clone();
        let token_with_key = format!("{}:{}", raw_token, secret_key);
        let secure_token_hash = self.hash_token(&token_with_key);

        let new_user_token = CreateRefreshToken::new(user_id, secure_token_hash);

        match self.repository.create(&new_user_token).await {
            Ok(refresh_token) => Ok(refresh_token),
            Err(err) => Err(RefreshTokenError::InternalServerError(err)),
        }
    }

    fn build_refresh_token_cookie(&self, raw_token: String) -> Result<Cookie, RefreshTokenError> {
        let refresh_validation_duration = *(constants::REFRESH_TOKEN_EXP_DAYS);
        let refresh_token_name = (constants::REFRESH_TOKEN_COOKIE_NAME).to_string();
        Ok(Cookie::build(refresh_token_name, raw_token.to_string())
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/")
            .max_age(Duration::days(refresh_validation_duration))
            .finish())
    }

    async fn renew_refresh_token(
        &self,
        raw_token: String,
        user_id: Uuid,
    ) -> Result<String, RefreshTokenError> {
        let refresh_token = self.get_refresh_token_from_raw_token(raw_token).await?;

        if refresh_token.is_revoked {
            match self
                .repository
                .revoke_family_id(refresh_token.family_id)
                .await
            {
                Ok(_) => {}
                Err(err) => return Err(RefreshTokenError::InternalServerError(err)),
            }
        }

        match self.repository.revoke_token(refresh_token.id).await {
            Ok(_) => {}
            Err(err) => return Err(RefreshTokenError::InternalServerError(err)),
        };
        let raw_token = self.create_raw_token();

        self.create_new_user_refresh_token(user_id, raw_token)
            .await?;
        Ok(raw_token.to_string())
    }

    async fn create_user_jwt_token(
        &self,
        user_id: Uuid,
        role: String,
    ) -> Result<String, RefreshTokenError> {
        let secret = (constants::JWT_SECRET).clone();

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
    async fn get_refresh_token_from_raw_token(
        &self,
        raw_token: String,
    ) -> Result<RefreshToken, RefreshTokenError> {
        let secret_key = (constants::REFRESH_TOKEN_SECRET).clone();
        let token_with_key = format!("{}:{}", raw_token, secret_key);
        let secure_token_hash = self.hash_token(&token_with_key);

        match self.repository.get_from_hash(secure_token_hash).await {
            Ok(Some(refresh_token)) => Ok(refresh_token),
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

    fn create_raw_token(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn verify_jwt(&self, token: &str) -> Result<JwtClaims, RefreshTokenError> {
        let secret = (constants::JWT_SECRET).clone();
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map_err(|err| RefreshTokenError::InternalServerError(err.into()))?;

        let now = Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(RefreshTokenError::TokenExpired);
        }

        Ok(token_data.claims)
    }
    fn extract_refresh_token(&self, req: &HttpRequest) -> Result<String, RefreshTokenError> {
        let refresh_token_cookie_name = (constants::REFRESH_TOKEN_COOKIE_NAME).clone();
        let cookie = req.cookie(&refresh_token_cookie_name);
        match cookie {
            Some(cookie) => Ok(cookie.value().to_string()),
            None => Err(RefreshTokenError::RefreshTokenDoesNotExist),
        }
    }

    fn extract_token(&self, req: &ServiceRequest) -> Result<String, RefreshTokenError> {
        let auth_header = match req.headers().get(AUTHORIZATION) {
            Some(auth_header) => auth_header,
            None => return Err(RefreshTokenError::JwtTokenDoesNotExist),
        };
        let auth_header = match auth_header.to_str() {
            Ok(auth_header) => auth_header,
            Err(_) => return Err(RefreshTokenError::InvalidTokenFormat),
        };

        if !auth_header.starts_with("Bearer ") {
            return Err(RefreshTokenError::InvalidTokenFormat);
        }

        Ok(auth_header[7..].to_string())
    }
}
