use todos_adapter::repository::health::HealthCheckRepository;
use todos_service::usecase::health::HealthCheckUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
}

impl Modules {
    pub async fn new() -> Self {
        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new());
        Self {
            health_check_use_case,
        }
    }
}
