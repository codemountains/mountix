use crate::model::surrounding_mountain::{
    SurroundingMountainDocument, SurroundingMountainFindCommand,
};
use crate::repository::MongoDBRepositoryImpl;
use async_trait::async_trait;
use futures::TryStreamExt;
use mountix_kernel::model::surrounding_mountain::{
    SurroundingMountain, SurroundingMountainSearchCondition,
};
use mountix_kernel::repository::surrounding_mountain::SurroundingMountainRepository;

#[async_trait]
impl SurroundingMountainRepository for MongoDBRepositoryImpl<SurroundingMountain> {
    async fn find(
        &self,
        search_condition: SurroundingMountainSearchCondition,
    ) -> anyhow::Result<Vec<SurroundingMountain>> {
        let collection = self
            .db
            .0
            .collection::<SurroundingMountainDocument>("mountains");

        let find_command: SurroundingMountainFindCommand = search_condition.try_into()?;
        let mut mountain_doc_list = collection.find(find_command.filter, None).await?;

        let mut mountains: Vec<SurroundingMountain> = Vec::new();
        while let Some(sd) = mountain_doc_list.try_next().await? {
            mountains.push(sd.try_into()?);
        }

        Ok(mountains)
    }
}
