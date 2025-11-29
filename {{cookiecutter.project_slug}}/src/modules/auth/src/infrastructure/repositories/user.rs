use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::repositories::user::UserRepository;
use base::result_paging::{QueryParams, RepositoryResult, ResultPaging};

pub struct UserDieselRepository {
    pub sqlx_pool: PgPool,
}

impl UserDieselRepository {
    pub fn new(sqlx_pool: PgPool) -> Self {
        UserDieselRepository { sqlx_pool }
    }
}

#[async_trait]
impl UserRepository for UserDieselRepository {
    async fn create(&self, new_item: &CreateUser) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                INSERT INTO users
                    (
                        id, email, phone_number, 
                        password_hash, first_name, last_name, 
                        username, role, is_active, is_verified, profile_image
                    ) 
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING *
            "#,
            Uuid::new_v4(),
            new_item.email,
            new_item.phone_number,
            new_item.password_hash,
            new_item.first_name,
            new_item.last_name,
            new_item.username,
            new_item.role,
            new_item.is_active,
            new_item.is_verified,
            new_item.profile_image,
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn update(&self, new_item: &UpdateUser) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                UPDATE users
                SET
                    email = $2, 
                    phone_number = $3, password_hash = $4,
                    first_name = $5, last_name = $6,
                    username = $7, role = $8, is_active = $9,
                    is_verified = $10, profile_image = $11 
                WHERE
                    id = $1
                RETURNING *
            "#,
            Uuid::new_v4(),
            new_item.email,
            new_item.phone_number,
            new_item.password_hash,
            new_item.first_name,
            new_item.last_name,
            new_item.username,
            new_item.role,
            new_item.is_active,
            new_item.is_verified,
            new_item.profile_image,
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn update_password(&self, new_password: String, user_id: Uuid) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                UPDATE users
                SET 
                    password_hash = $2 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            user_id,
            new_password,
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn set_new_otp(
        &self,
        user_id: Uuid,
        otp: String,
        otp_expiry: DateTime<Utc>,
    ) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                UPDATE users
                SET 
                    otp = $2,
                    otp_expiry = $3,
                    otp_attempts = 0 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            user_id,
            otp,
            otp_expiry
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn increment_otp_attempts(&self, user_id: Uuid) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                UPDATE users
                SET 
                    otp_attempts = otp_attempts + 1 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            user_id,
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn reset_otp_attempts(&self, user_id: Uuid) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                UPDATE users
                SET 
                    otp_attempts = $2 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            user_id,
            0,
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn expire_otp(&self, user_id: Uuid) -> RepositoryResult<User> {
        Ok(query_as!(
            User,
            r#"
                UPDATE users
                SET 
                    otp_expiry = $2 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            user_id,
            Utc::now(),
        )
        .fetch_one(&self.sqlx_pool)
        .await?)
    }

    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>> {
        let result = query_as!(
            User,
            r#"
                SELECT *
                FROM users
                LIMIT $1 OFFSET $2
            "#,
            params.limit(),
            params.offset()
        )
        .fetch_all(&self.sqlx_pool)
        .await?;

        let total = query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM users
        "#
        )
        .fetch_one(&self.sqlx_pool)
        .await?
        .count;

        Ok(ResultPaging {
            total,
            items: result,
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<User>> {
        Ok(query_as!(
            User,
            r#"
                SELECT * FROM users 
                WHERE
                id = $1
            "#,
            item_id
        )
        .fetch_optional(&self.sqlx_pool)
        .await?)
    }

    async fn get_by_email(&self, user_email: String) -> RepositoryResult<Option<User>> {
        Ok(query_as!(
            User,
            r#"
                SELECT * FROM users 
                WHERE
                email = $1
            "#,
            user_email
        )
        .fetch_optional(&self.sqlx_pool)
        .await?)
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
                DELETE FROM users WHERE id = $1
            "#,
            item_id
        )
        .execute(&self.sqlx_pool)
        .await?;
        Ok(())
    }
}
