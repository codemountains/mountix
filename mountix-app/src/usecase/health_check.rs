use mountix_adapter::repository::health_check::HealthCheckRepository;
use std::sync::Arc;

pub struct HealthCheckUseCase {
    repository: Arc<HealthCheckRepository>,
}

impl HealthCheckUseCase {
    pub fn new(repository: HealthCheckRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn diagnose_mongo_db_conn(&self) -> anyhow::Result<()> {
        self.repository.check_mongo_db().await
    }
}
