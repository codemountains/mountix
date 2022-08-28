use crate::{persistence::mongodb::Db, repository::MongoDBRepositoryImpl};
use mountix_kernel::model::surrounding_mountain::SurroundingMountain;
use mountix_kernel::repository::surrounding_mountain::SurroundingMountainRepository;
use mountix_kernel::{model::mountain::Mountain, repository::mountain::MountainRepository};

pub struct RepositoriesModule {
    mountain_repository: MongoDBRepositoryImpl<Mountain>,
    surrounding_mountain_repository: MongoDBRepositoryImpl<SurroundingMountain>,
}

pub trait RepositoriesModuleExt {
    type MountainRepo: MountainRepository;
    type SurroundingMountainRepo: SurroundingMountainRepository;

    fn mountain_repository(&self) -> &Self::MountainRepo;
    fn surrounding_mountain_repository(&self) -> &Self::SurroundingMountainRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type MountainRepo = MongoDBRepositoryImpl<Mountain>;
    type SurroundingMountainRepo = MongoDBRepositoryImpl<SurroundingMountain>;

    fn mountain_repository(&self) -> &Self::MountainRepo {
        &self.mountain_repository
    }
    fn surrounding_mountain_repository(&self) -> &Self::SurroundingMountainRepo {
        &self.surrounding_mountain_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let mountain_repository = MongoDBRepositoryImpl::new(db.clone());
        let surrounding_mountain_repository = MongoDBRepositoryImpl::new(db.clone());
        Self {
            mountain_repository,
            surrounding_mountain_repository,
        }
    }
}
