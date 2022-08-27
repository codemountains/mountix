use crate::model::surrounding_mountain::{SurroundingMountain, SurroundingMountainSearchCondition};
use async_trait::async_trait;

#[async_trait]
pub trait SurroundingMountainRepository {
    async fn find(
        &self,
        search_condition: SurroundingMountainSearchCondition,
    ) -> anyhow::Result<Vec<SurroundingMountain>>;
}
