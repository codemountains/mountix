use mountix_adapter::modules::{RepositoriesModule, RepositoriesModuleExt};
use mountix_adapter::persistence::mongodb::Db;
use mountix_adapter::repository::health_check::HealthCheckRepository;
use mountix_app::usecase::health_check::HealthCheckUseCase;
use mountix_app::usecase::mountain::MountainUseCase;
use std::sync::Arc;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
    mountain_use_case: MountainUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase;
    fn mountain_use_case(&self) -> &MountainUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }

    fn mountain_use_case(&self) -> &MountainUseCase<Self::RepositoriesModule> {
        &self.mountain_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new(db));
        let mountain_use_case = MountainUseCase::new(repositories_module.clone());

        Self {
            health_check_use_case,
            mountain_use_case,
        }
    }
}
