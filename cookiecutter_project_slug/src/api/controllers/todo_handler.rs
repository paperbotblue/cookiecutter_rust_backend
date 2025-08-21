use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::todo::{CreateTodoDTO, TodoDTO, UpdateTodoDTO};
use crate::api::dto::wapper_uuid::UuidParam;
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;
use crate::domain::services::todo::TodoService;

pub async fn create_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    post_data: web::Json<CreateTodoDTO>,
) -> Result<web::Json<TodoDTO>, ApiError> {
    let todo = todo_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(todo.into()))
}

pub async fn update_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    post_data: web::Json<UpdateTodoDTO>,
) -> Result<web::Json<TodoDTO>, ApiError> {
    let todo = todo_service.update(post_data.into_inner().into()).await?;
    Ok(web::Json(todo.into()))
}

pub async fn list_todos_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Query<TodoQueryParams>,
) -> Result<web::Json<ResultPaging<TodoDTO>>, ApiError> {
    let selection = todo_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    params: UuidParam,
) -> Result<web::Json<TodoDTO>, ApiError> {
    let todo = todo_service.get(params.0).await?;
    Ok(web::Json(todo.into()))
}

pub async fn delete_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    params: UuidParam,
) -> Result<HttpResponse, ApiError> {
    todo_service.delete(params.0).await?;
    Ok(HttpResponse::NoContent().finish())
}
