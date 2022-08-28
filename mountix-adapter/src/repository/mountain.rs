use crate::model::mountain::{MountainDocument, MountainFindBoxCommand, MountainFindCommand};
use crate::repository::MongoDBRepositoryImpl;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mountix_kernel::model::mountain::{
    Mountain, MountainBoxSearchCondition, MountainSearchCondition,
};
use mountix_kernel::model::Id;
use mountix_kernel::repository::mountain::MountainRepository;

#[async_trait]
impl MountainRepository for MongoDBRepositoryImpl<Mountain> {
    async fn get(&self, id: Id<Mountain>) -> anyhow::Result<Option<Mountain>> {
        let collection = self.db.0.collection::<MountainDocument>("mountains");

        let filter = doc! {"_id": id.value};
        let mountain_doc = collection.find_one(filter, None).await?;
        match mountain_doc {
            Some(md) => Ok(Some(md.try_into()?)),
            None => Ok(None),
        }
    }

    async fn get_count(&self, search_condition: MountainSearchCondition) -> anyhow::Result<u64> {
        let collection = self.db.0.collection::<MountainDocument>("mountains");
        let find_command: MountainFindCommand = search_condition.try_into()?;
        let count = collection
            .count_documents(find_command.filter, None)
            .await?;
        Ok(count)
    }

    async fn find(
        &self,
        search_condition: MountainSearchCondition,
    ) -> anyhow::Result<Vec<Mountain>> {
        let collection = self.db.0.collection::<MountainDocument>("mountains");

        let find_command: MountainFindCommand = search_condition.try_into()?;
        let mut mountain_doc_list = collection
            .find(find_command.filter, find_command.options)
            .await?;

        let mut mountains: Vec<Mountain> = Vec::new();
        while let Some(md) = mountain_doc_list.try_next().await? {
            mountains.push(md.try_into()?);
        }

        Ok(mountains)
    }

    async fn find_box(
        &self,
        search_condition: MountainBoxSearchCondition,
    ) -> anyhow::Result<Vec<Mountain>> {
        let collection = self.db.0.collection::<MountainDocument>("mountains");

        let find_command: MountainFindBoxCommand = search_condition.try_into()?;
        let mut mountain_doc_list = collection
            .find(find_command.filter, find_command.options)
            .await?;

        let mut mountains: Vec<Mountain> = Vec::new();
        while let Some(md) = mountain_doc_list.try_next().await? {
            mountains.push(md.try_into()?);
        }

        Ok(mountains)
    }
}
