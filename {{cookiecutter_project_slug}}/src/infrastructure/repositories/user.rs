use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::user::{CreateUserDiesel, UpdateUserDiesel, UserDiesel};

pub struct UserDieselRepository {
    pub pool: Arc<DBConn>,
}

impl UserDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        UserDieselRepository { pool: db }
    }
}

#[async_trait]
impl UserRepository for UserDieselRepository {
    async fn create(&self, new_item: &CreateUser) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::users;
        let new_item_diesel: CreateUserDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: UserDiesel = run(move || {
            diesel::insert_into(users)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: &UpdateUser) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::{id as target_id, users};
        let new_item_diesel: UpdateUserDiesel = new_item.into();
        let id_val: Uuid = new_item.id;
        let mut conn = self.pool.get().unwrap();
        let result: UserDiesel = run(move || {
            diesel::update(users.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>> {
        use crate::infrastructure::schema::users::dsl::users;
        let pool = self.pool.clone();
        let builder = users.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<UserDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<User>> {
        use crate::infrastructure::schema::users::dsl::{id, users};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            users
                .filter(id.eq(item_id))
                .first::<UserDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into()))
    }

    async fn get_by_email(&self, item_id: String) -> RepositoryResult<Option<User>> {
        use crate::infrastructure::schema::users::dsl::{email, users};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            users
                .filter(email.eq(item_id))
                .first::<UserDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into()))
    }

    async fn get_by_phone_number(&self, item_id: String) -> RepositoryResult<Option<User>> {
        use crate::infrastructure::schema::users::dsl::{phone_number, users};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            users
                .filter(phone_number.eq(item_id))
                .first::<UserDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into()))
    }

    async fn get_by_username(&self, item_id: String) -> RepositoryResult<Option<User>> {
        use crate::infrastructure::schema::users::dsl::{username, users};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            users
                .filter(username.eq(item_id))
                .first::<UserDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into()))
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::users::dsl::{id, users};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(users.filter(id.eq(item_id))).execute(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }
}
