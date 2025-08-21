use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::todo::{CreateTodo, Todo, UpdateTodo};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;

#[async_trait]
pub trait TodoService: 'static + Sync + Send {
    async fn create(&self, todo: CreateTodo) -> Result<Todo, CommonError>;
    async fn update(&self, todo: UpdateTodo) -> Result<Todo, CommonError>;
    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError>;
    async fn get(&self, todo_id: Uuid) -> Result<Todo, CommonError>;
    async fn delete(&self, todo_id: Uuid) -> Result<(), CommonError>;
}
