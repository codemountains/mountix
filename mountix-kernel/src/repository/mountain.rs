use crate::model::Id;
use async_trait::async_trait;

use crate::model::mountain::{Mountain, MountainSearchCondition};

#[async_trait]
pub trait MountainRepository {
    async fn get(&self, id: Id<Mountain>) -> anyhow::Result<Option<Mountain>>;
    async fn get_count(&self, search_condition: MountainSearchCondition) -> anyhow::Result<u64>;
    async fn find(
        &self,
        search_condition: MountainSearchCondition,
    ) -> anyhow::Result<Vec<Mountain>>;
}
