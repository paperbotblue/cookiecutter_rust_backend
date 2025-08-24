use diesel::prelude::*;
use crate::domain::models::todo::{CreateTodo, Todo, UpdateTodo};
use crate::infrastructure::schema::todos;
use uuid::Uuid;

#[derive(Queryable)]
pub struct TodoDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<Todo> for TodoDiesel {
    fn from(t: Todo) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct CreateTodoDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = todos)]
pub struct UpdateTodoDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl From<TodoDiesel> for Todo {
    fn from(d: TodoDiesel) -> Self {
        Self {
            id: d.id,
            name: d.name,
            description: d.description,
        }
    }
}

impl From<CreateTodo> for CreateTodoDiesel {
    fn from(t: CreateTodo) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: t.name,
            description: t.description,
        }
    }
}

impl From<&CreateTodo> for CreateTodoDiesel {
    fn from(t: &CreateTodo) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: t.name.clone(),
            description: t.description.clone(),
        }
    }
}

impl From<UpdateTodo> for UpdateTodoDiesel {
    fn from(t: UpdateTodo) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
        }
    }
}

impl From<&UpdateTodo> for UpdateTodoDiesel {
    fn from(t: &UpdateTodo) -> Self {
        Self {
            id: t.id,
            name: t.name.clone(),
            description: t.description.clone(),
        }
    }
}

impl From<CreateTodoDiesel> for Todo {
    fn from(d: CreateTodoDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: d.name,
            description: d.description,
        }
    }
}

