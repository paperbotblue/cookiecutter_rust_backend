use crate::infrastructure::databases::postgresql::db_pool;
use std::sync::Arc;

pub struct Container {
    //pub todo_service: Arc<dyn TodoService>,
}

impl Container {
    pub fn new() -> Self {
        let pool = Arc::new(db_pool());

        //let todo_repository: Arc<dyn TodoRepository> =
        //    Arc::new(TodoDieselRepository::new(pool.clone()));
        //let todo_service = Arc::new(TodoServiceImpl {
        //    repository: todo_repository,
        //});

        Container { /* todo_service */ }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
