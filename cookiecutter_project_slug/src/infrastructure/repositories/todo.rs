use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::todo::{CreateTodo, Todo, UpdateTodo};
use crate::domain::repositories::todo::TodoQueryParams;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::todo::{CreateTodoDiesel, TodoDiesel, UpdateTodoDiesel};

pub struct TodoDieselRepository {
    pub pool: Arc<DBConn>,
}

impl TodoDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TodoDieselRepository { pool: db }
    }
}

#[async_trait]
impl TodoRepository for TodoDieselRepository {
    async fn create(&self, new_item: &CreateTodo) -> RepositoryResult<Todo> {
        use crate::infrastructure::schema::todos::dsl::todos;
        let new_item_diesel: CreateTodoDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: TodoDiesel = run(move || {
            diesel::insert_into(todos)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: &UpdateTodo) -> RepositoryResult<Todo> {
        use crate::infrastructure::schema::todos::dsl::{id as target_id, todos};
        let new_item_diesel: UpdateTodoDiesel = new_item.into();
        let id_val: Uuid = new_item.id;
        let mut conn = self.pool.get().unwrap();
        let result: TodoDiesel = run(move || {
            diesel::update(todos.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>> {
        use crate::infrastructure::schema::todos::dsl::todos;
        let pool = self.pool.clone();
        let builder = todos.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<TodoDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Todo> {
        use crate::infrastructure::schema::todos::dsl::{id, todos};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            todos
                .filter(id.eq(item_id))
                .first::<TodoDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|v| v.into())
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::todos::dsl::{id, todos};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(todos.filter(id.eq(item_id))).execute(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }
}

