use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::ApiError;
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
    async fn create(&self, item: CreateTodo) -> Result<Todo, ApiError> {
        self.repository
            .create(&item)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn update(&self, item: UpdateTodo) -> Result<Todo, ApiError> {
        self.repository
            .update(&item)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, ApiError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn get(&self, item_id: Uuid) -> Result<Todo, ApiError> {
        self.repository
            .get(item_id)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), ApiError> {
        self.repository
            .delete(item_id)
            .await
            .map_err(|e| TodoError::InternalServerError(e.message.to_string()).into())
    }
}
