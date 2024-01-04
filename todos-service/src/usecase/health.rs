use std::sync::Arc;

use todos_adapter::repository::health::HealthCheckRepository;

pub struct HealthCheckUseCase {
    repository: Arc<HealthCheckRepository>,
}

impl HealthCheckUseCase {
    pub fn new(repository: HealthCheckRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}
