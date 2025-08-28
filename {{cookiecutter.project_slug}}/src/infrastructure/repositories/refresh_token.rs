use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::refresh_token::{CreateRefreshToken, RefreshToken, UpdateRefreshToken};
use crate::domain::repositories::refresh_token::RefreshTokenQueryParams;
use crate::domain::repositories::refresh_token::RefreshTokenRepository;
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::refresh_token::{
    CreateRefreshTokenDiesel, RefreshTokenDiesel, UpdateRefreshTokenDiesel,
};

pub struct RefreshTokenDieselRepository {
    pub pool: Arc<DBConn>,
}

impl RefreshTokenDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        RefreshTokenDieselRepository { pool: db }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenDieselRepository {
    async fn create(&self, new_item: &CreateRefreshToken) -> RepositoryResult<RefreshToken> {
        use crate::infrastructure::schema::refresh_tokens::dsl::refresh_tokens;
        let new_item_diesel: CreateRefreshTokenDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: RefreshTokenDiesel = run(move || {
            diesel::insert_into(refresh_tokens)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: &UpdateRefreshToken) -> RepositoryResult<RefreshToken> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{id as target_id, refresh_tokens};
        let new_item_diesel: UpdateRefreshTokenDiesel = new_item.into();
        let id_val: Uuid = new_item.id;
        let mut conn = self.pool.get().unwrap();
        let result: RefreshTokenDiesel = run(move || {
            diesel::update(refresh_tokens.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn revoke_family_id(&self, id_val: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{
            family_id, is_revoked, refresh_tokens,
        };

        let mut conn = self.pool.get().unwrap();

        run(move || {
            diesel::update(refresh_tokens.filter(family_id.eq(id_val)))
                .set(is_revoked.eq(true))
                .execute(&mut conn) // <- use execute for multiple rows
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(())
    }

    async fn revoke_token(&self, id_val: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{id, is_revoked, refresh_tokens};

        let mut conn = self.pool.get().unwrap();

        run(move || {
            diesel::update(refresh_tokens.filter(id.eq(id_val)))
                .set(is_revoked.eq(true))
                .execute(&mut conn) // <- use execute for multiple rows
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(())
    }

    async fn list(
        &self,
        params: RefreshTokenQueryParams,
    ) -> RepositoryResult<ResultPaging<RefreshToken>> {
        use crate::infrastructure::schema::refresh_tokens::dsl::refresh_tokens;
        let pool = self.pool.clone();
        let builder = refresh_tokens.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<RefreshTokenDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<RefreshToken>> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            refresh_tokens
                .filter(id.eq(item_id))
                .first::<RefreshTokenDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into())) // map over Option
    }

    async fn get_from_hash(&self, item_id: String) -> RepositoryResult<Option<RefreshToken>> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{refresh_tokens, token};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            refresh_tokens
                .filter(token.eq(item_id))
                .first::<RefreshTokenDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into())) // map over Option
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(refresh_tokens.filter(id.eq(item_id))).execute(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }
}
