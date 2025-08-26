use crate::{
    api::dto::todo::{CreateTodoDTO, TodoDTO, UpdateTodoDTO},
    domain::{
        models::todo::{CreateTodo, Todo, UpdateTodo},
        repositories::repository::ResultPaging,
    },
};

impl From<Todo> for TodoDTO {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

impl From<CreateTodoDTO> for CreateTodo {
    fn from(value: CreateTodoDTO) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

impl From<UpdateTodoDTO> for UpdateTodo {
    fn from(value: UpdateTodoDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

impl From<CreateTodo> for CreateTodoDTO {
    fn from(value: CreateTodo) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

impl From<UpdateTodo> for UpdateTodoDTO {
    fn from(value: UpdateTodo) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

impl From<ResultPaging<Todo>> for ResultPaging<TodoDTO> {
    fn from(value: ResultPaging<Todo>) -> Self {
        Self {
            total: value.total,
            items: value.items.into_iter().map(Into::into).collect(),
        }
    }
}
