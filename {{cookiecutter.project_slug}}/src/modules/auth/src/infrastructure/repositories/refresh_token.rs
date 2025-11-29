use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use base::result_paging::{QueryParams, RepositoryResult, ResultPaging};
pub struct RefreshTokenDieselRepository {
    pub pool: PgPool,
}

impl RefreshTokenDieselRepository {
    pub fn new(db: PgPool) -> Self {
        RefreshTokenDieselRepository { pool: db }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenDieselRepository {
    async fn revoke_family_id(&self, id_val: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
            UPDATE refresh_tokens
            SET
                is_revoked = true
            WHERE
                family_id = $1
        "#,
            id_val
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn revoke_token(&self, id_val: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
                UPDATE refresh_tokens 
                SET 
                    is_revoked = true 
                WHERE id = $1
            "#,
            id_val,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_from_hash(&self, item_id: String) -> RepositoryResult<Option<RefreshToken>> {
        Ok(query_as!(
            RefreshToken,
            r#"
                SELECT * FROM refresh_tokens WHERE token = $1
            "#,
            item_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn create(&self, new_item: &CreateRefreshToken) -> RepositoryResult<RefreshToken> {
        Ok(query_as!(
            RefreshToken,
            r#"
                INSERT INTO refresh_tokens
                    (
                        id, user_id, token, family_id,
                        issued_at, expires_at, is_revoked
                    ) 
                VALUES
                    (
                        $1, $2, $3, $4,
                        $5, $6, $7
                    )
                RETURNING *
            "#,
            Uuid::new_v4(),
            new_item.user_id,
            new_item.token,
            new_item.family_id,
            new_item.issued_at,
            new_item.expires_at,
            new_item.is_revoked,
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update(&self, new_item: &UpdateRefreshToken) -> RepositoryResult<RefreshToken> {
        Ok(query_as!(
            RefreshToken,
            r#"
                UPDATE refresh_tokens
                SET
                    user_id    = $2, 
                    token      = $3, 
                    family_id  = $4, 
                    issued_at  = $5, 
                    expires_at = $6, 
                    is_revoked = $7 
                WHERE 
                    id = $1
                RETURNING *
            "#,
            Uuid::new_v4(),
            new_item.user_id,
            new_item.token,
            new_item.family_id,
            new_item.issued_at,
            new_item.expires_at,
            new_item.is_revoked,
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn list(
        &self,
        params: RefreshTokenQueryParams,
    ) -> RepositoryResult<ResultPaging<RefreshToken>> {
        let result = query_as!(
            RefreshToken,
            r#"
                SELECT * FROM refresh_tokens LIMIT $1 OFFSET $2
            "#,
            params.limit(),
            params.offset()
        )
        .fetch_all(&self.pool)
        .await?;

        let total = query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM refresh_tokens
        "#
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        Ok(ResultPaging {
            total,
            items: result,
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<RefreshToken>> {
        Ok(query_as!(
            RefreshToken,
            r#"
                SELECT * FROM refresh_tokens 
                WHERE 
                id = $1
            "#,
            item_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
            DELETE FROM refresh_tokens 
            WHERE 
            id = $1
        "#,
            item_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
