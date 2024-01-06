use std::sync::Arc;

use todos_adapter::repository::todo::TodoRepositoryForMemory;
use todos_app::usecase::todo::TodoUseCase;

// module is a collection of use cases
pub struct Modules {
    todo_use_case: TodoUseCase<TodoRepositoryForMemory>,
}

impl Modules {
    pub async fn new() -> Self {
        let todo_use_case = TodoUseCase::new(Arc::new(TodoRepositoryForMemory::new()));
        Self { todo_use_case }
    }

    pub fn todo_use_case(&self) -> &TodoUseCase<TodoRepositoryForMemory> {
        &self.todo_use_case
    }
}
