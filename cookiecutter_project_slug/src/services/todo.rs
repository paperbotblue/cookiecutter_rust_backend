use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::errors::todo_errors::TodoError;
use crate::domain::models::todo::{CreateTodo, Todo, UpdateTodo};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::services::todo::TodoService;

#[derive(Clone)]
pub struct TodoServiceImpl {
    pub repository: Arc<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn create(&self, mut item: CreateTodo) -> Result<Todo, CommonError> {
        self.repository
            .create(&mut item)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn update(&self, mut item: UpdateTodo) -> Result<Todo, CommonError> {
        self.repository
            .update(&mut item)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn get(&self, item_id: Uuid) -> Result<Todo, CommonError> {
        self.repository
            .get(item_id)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), CommonError> {
        self.repository
            .delete(item_id)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }
}
