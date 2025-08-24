use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::ApiError;
use crate::domain::models::todo::{CreateTodo, Todo, UpdateTodo};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;

#[async_trait]
pub trait TodoService: 'static + Sync + Send {
    async fn create(&self, todo: CreateTodo) -> Result<Todo, ApiError>;
    async fn update(&self, todo: UpdateTodo) -> Result<Todo, ApiError>;
    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, ApiError>;
    async fn get(&self, todo_id: Uuid) -> Result<Todo, ApiError>;
    async fn delete(&self, todo_id: Uuid) -> Result<(), ApiError>;
}
