use async_trait::async_trait;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::dto::user::LoginDTO;
use crate::domain::errors::user_errors::UserError;
use crate::domain::models::user::{CreateUser, UpdatePassword, UpdateUser, User};
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;
use crate::infrastructure::repositories::user::UserDieselRepository;
use base::constants;
use base::result_paging::ResultPaging;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: Arc::new(UserDieselRepository::new(pool)),
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, mut item: CreateUser) -> Result<User, UserError> {
        item.password_hash = self.hash_password(&item.password_hash);

        match self.repository.create(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn login(&self, item: LoginDTO) -> Result<User, UserError> {
        match self.repository.get_by_email(item.email.clone()).await {
            Ok(Some(user)) => {
                if self.verify_password(&item.password, &user.password_hash) {
                    Ok(user)
                } else {
                    Err(UserError::UserNotAuthorised)
                }
            }
            Ok(None) => Err(UserError::UserDoesNotExist),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn update(&self, item: UpdateUser) -> Result<User, UserError> {
        match self.repository.update(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }
    async fn update_password(
        &self,
        password: UpdatePassword,
        user_id: Uuid,
    ) -> Result<(), UserError> {
        match self.repository.get(user_id).await {
            Ok(Some(user)) => {
                if let Some(otp_expiry) = user.otp_expiry {
                    if otp_expiry <= Utc::now() {
                        return Err(UserError::UserOtpExpired);
                    }
                    if user.otp_attempts > Some(3) {
                        return Err(UserError::UserOtpAttemptsExceeded);
                    }
                    if user.otp != Some(password.otp) {
                        self.repository
                            .increment_otp_attempts(user_id)
                            .await
                            .map_err(UserError::InternalServerError)?;
                        return Err(UserError::UserOtpIncorrect);
                    } else {
                        self.repository
                            .reset_otp_attempts(user_id)
                            .await
                            .map_err(UserError::InternalServerError)?;
                        self.repository
                            .expire_otp(user_id)
                            .await
                            .map_err(UserError::InternalServerError)?;
                        self.repository
                            .update_password(self.hash_password(&password.password), user_id)
                            .await
                            .map_err(UserError::InternalServerError)?;
                        return Ok(());
                    }
                }
                return Ok(());
            }
            Ok(None) => return Err(UserError::UserDoesNotExist),
            Err(err) => return Err(UserError::InternalServerError(err)),
        }
    }

    async fn send_otp(&self, user_id: Uuid, otp: String) -> Result<(), UserError> {
        match self.repository.get(user_id).await {
            Ok(Some(user)) => {
                if user.otp_expiry > Some(Utc::now()) {
                    return Err(UserError::UserOtpNotExpired);
                }
            }
            Ok(None) => {}
            Err(err) => return Err(UserError::InternalServerError(err)),
        };
        let new_otp_expiry = Utc::now() + Duration::minutes(*constants::OTP_EXP_MINUTES);
        self.repository
            .set_new_otp(user_id, otp, new_otp_expiry)
            .await
            .map_err(UserError::InternalServerError)?;
        Ok(())
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

    fn hash_password(&self, token: &str) -> String {
        let secret_key = (constants::PASSWORD_HASH_SECRET).clone();
        let token_with_key = format!("{}:{}", token, secret_key);
        let mut hasher = Sha256::new();
        hasher.update(token_with_key.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        self.hash_password(password) == password_hash
    }
}
