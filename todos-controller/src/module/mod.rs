use std::sync::Arc;

use todos_adapter::repository::{health::HealthCheckRepository, todo::TodoRepositoryForMemory};
use todos_app::usecase::{health::HealthCheckUseCase, todo::TodoUseCase};

// module is a collection of use cases
pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
    todo_use_case: TodoUseCase<TodoRepositoryForMemory>,
}

impl Modules {
    pub async fn new() -> Self {
        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new());
        let todo_use_case = TodoUseCase::new(Arc::new(TodoRepositoryForMemory::new()));
        Self {
            health_check_use_case,
            todo_use_case,
        }
    }

    pub fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }

    pub fn todo_use_case(&self) -> &TodoUseCase<TodoRepositoryForMemory> {
        &self.todo_use_case
    }
}
