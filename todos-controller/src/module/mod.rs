use todos_adapter::repository::{health::HealthCheckRepository, todo::TodoRepository};
use todos_service::usecase::{health::HealthCheckUseCase, todo::TodoUseCase};

// module is a collection of use cases
pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
    todo_use_case: TodoUseCase,
}

impl Modules {
    pub async fn new() -> Self {
        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new());
        let todo_use_case = TodoUseCase::new(TodoRepository::new());
        Self {
            health_check_use_case,
            todo_use_case,
        }
    }
}
