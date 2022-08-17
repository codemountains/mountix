use crate::{persistence::mongodb::Db, repository::MongoDBRepositoryImpl};
use mountix_kernel::{model::mountain::Mountain, repository::mountain::MountainRepository};

pub struct RepositoriesModule {
    mountain_repository: MongoDBRepositoryImpl<Mountain>,
}

pub trait RepositoriesModuleExt {
    type MountainRepo: MountainRepository;
    fn mountain_repository(&self) -> &Self::MountainRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type MountainRepo = MongoDBRepositoryImpl<Mountain>;
    fn mountain_repository(&self) -> &Self::MountainRepo {
        &self.mountain_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let mountain_repository = MongoDBRepositoryImpl::new(db.clone());
        Self {
            mountain_repository,
        }
    }
}
