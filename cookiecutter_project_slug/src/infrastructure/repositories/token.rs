use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::token::{CreateToken, Token, UpdateToken};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::token::TokenQueryParams;
use crate::domain::repositories::token::TokenRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::token::{CreateTokenDiesel, TokenDiesel, UpdateTokenDiesel};

pub struct TokenDieselRepository {
    pub pool: Arc<DBConn>,
}

impl TokenDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TokenDieselRepository { pool: db }
    }
}

#[async_trait]
impl TokenRepository for TokenDieselRepository {
    async fn create(&self, new_item: &CreateToken) -> RepositoryResult<Token> {
        use crate::infrastructure::schema::refresh_tokens::dsl::refresh_tokens;
        let new_item_diesel: CreateTokenDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: TokenDiesel = run(move || {
            diesel::insert_into(refresh_tokens)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: &UpdateToken) -> RepositoryResult<Token> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{id as target_id, refresh_tokens};
        let new_item_diesel: UpdateTokenDiesel = new_item.into();
        let id_val: Uuid = new_item.id;
        let mut conn = self.pool.get().unwrap();
        let result: TokenDiesel = run(move || {
            diesel::update(refresh_tokens.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: TokenQueryParams) -> RepositoryResult<ResultPaging<Token>> {
        use crate::infrastructure::schema::refresh_tokens::dsl::refresh_tokens;
        let pool = self.pool.clone();
        let builder = refresh_tokens.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<TokenDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<Token>> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{id, refresh_tokens};
        let mut conn = self.pool.get().unwrap();
        let result = run(move || {
            refresh_tokens
                .filter(id.eq(item_id))
                .first::<TokenDiesel>(&mut conn)
        })
        .await;

        match result {
            Ok(v) => Ok(Some(v.into())),
            Err(actix_threadpool::BlockingError::Error(DieselError::NotFound)) => Ok(None),
            Err(e) => Err(DieselRepositoryError::from(e).into_inner()),
        }
    }
    async fn get_by_client_id(&self, item_id: Uuid) -> RepositoryResult<Option<Token>> {
        use crate::infrastructure::schema::refresh_tokens::dsl::{client_id, refresh_tokens};
        let mut conn = self.pool.get().unwrap();
        let result = run(move || {
            refresh_tokens
                .filter(client_id.eq(item_id))
                .first::<TokenDiesel>(&mut conn)
        })
        .await;

        match result {
            Ok(v) => Ok(Some(v.into())),
            Err(actix_threadpool::BlockingError::Error(DieselError::NotFound)) => Ok(None),
            Err(e) => Err(DieselRepositoryError::from(e).into_inner()),
        }
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
