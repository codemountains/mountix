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
        let mountain_doc = collection.find_one(filter).await?;
        match mountain_doc {
            Some(md) => Ok(Some(md.try_into()?)),
            None => Ok(None),
        }
    }

    async fn get_count(&self, search_condition: MountainSearchCondition) -> anyhow::Result<u64> {
        let collection = self.db.0.collection::<MountainDocument>("mountains");
        let find_command: MountainFindCommand = search_condition.try_into()?;
        let count = collection.count_documents(find_command.filter).await?;
        Ok(count)
    }

    async fn find(
        &self,
        search_condition: MountainSearchCondition,
    ) -> anyhow::Result<Vec<Mountain>> {
        let collection = self.db.0.collection::<MountainDocument>("mountains");

        let find_command: MountainFindCommand = search_condition.try_into()?;
        let mut mountain_doc_list = collection
            .find(find_command.filter)
            .with_options(find_command.options)
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
            .find(find_command.filter)
            .with_options(find_command.options)
            .await?;

        let mut mountains: Vec<Mountain> = Vec::new();
        while let Some(md) = mountain_doc_list.try_next().await? {
            mountains.push(md.try_into()?);
        }

        Ok(mountains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::mountain::MountainLocationDocument;
    use mountix_kernel::model::mountain::{
        MountainPrefecture, MountainSearchCondition, MountainSortCondition, MountainTag,
    };

    fn create_test_mountain_document() -> MountainDocument {
        MountainDocument {
            id: 1,
            name: "富士山".to_string(),
            name_kana: "ふじさん".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string(), "山梨県".to_string()],
            elevation: 3776,
            tags: vec!["百名山".to_string()],
            location: MountainLocationDocument {
                r#type: "Point".to_string(),
                coordinates: [138.727778, 35.360556], // [longitude, latitude] - MongoDB形式
            },
            gsi_url: "https://maps.gsi.go.jp/fuji".to_string(),
        }
    }

    #[test]
    fn test_mountain_document_to_mountain_conversion() {
        let mountain_doc = create_test_mountain_document();
        let result = Mountain::try_from(mountain_doc);

        assert!(result.is_ok());
        let mountain = result.unwrap();
        assert_eq!(mountain.id.value, 1);
        assert_eq!(mountain.name, "富士山");
        assert_eq!(mountain.name_kana, "ふじさん");
        assert_eq!(mountain.area, "関東地方");
        assert_eq!(mountain.prefectures, vec!["静岡県", "山梨県"]);
        assert_eq!(mountain.elevation, 3776);
        assert_eq!(mountain.location.latitude, 35.360556);
        assert_eq!(mountain.location.longitude, 138.727778);
        assert_eq!(mountain.location.gsi_url, "https://maps.gsi.go.jp/fuji");
        assert_eq!(mountain.tags, vec!["百名山"]);
    }

    #[test]
    fn test_mountain_search_condition_to_find_command() {
        let search_condition = MountainSearchCondition {
            name: Some("富士".to_string()),
            prefecture: Some(MountainPrefecture::try_from("19".to_string()).unwrap()),
            tag: Some(MountainTag::try_from("1".to_string()).unwrap()),
            skip: 10,
            limit: Some(5),
            sort: MountainSortCondition::default(),
        };

        let result = MountainFindCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        assert!(command.filter.contains_key("$and"));
        assert_eq!(command.options.skip, Some(10));
        assert_eq!(command.options.limit, Some(5));
    }

    #[test]
    fn test_mountain_box_search_condition_to_find_command() {
        let box_coords = mountix_kernel::model::mountain::MountainBoxCoordinates::try_from(
            "(138.0,35.0),(139.0,36.0)".to_string(),
        )
        .unwrap();
        let search_condition = mountix_kernel::model::mountain::MountainBoxSearchCondition {
            box_coordinates: box_coords,
            name: Some("富士".to_string()),
            tag: Some(MountainTag::try_from("1".to_string()).unwrap()),
            sort: MountainSortCondition::default(),
        };

        let result = MountainFindBoxCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        assert!(command.filter.contains_key("$and"));
        assert!(
            command
                .filter
                .get("$and")
                .unwrap()
                .as_array()
                .unwrap()
                .len()
                >= 2
        );
    }

    #[test]
    fn test_mountain_search_condition_empty_filters() {
        let search_condition = MountainSearchCondition {
            name: None,
            prefecture: None,
            tag: None,
            skip: 0,
            limit: None,
            sort: MountainSortCondition::default(),
        };

        let result = MountainFindCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        assert!(!command.filter.contains_key("$and"));
        assert_eq!(command.options.skip, Some(0));
        assert_eq!(command.options.limit, None);
    }

    #[test]
    fn test_mountain_sort_condition_elevation_desc() {
        let sort_condition = MountainSortCondition::try_from("elevation.desc".to_string()).unwrap();
        let search_condition = MountainSearchCondition {
            name: None,
            prefecture: None,
            tag: None,
            skip: 0,
            limit: None,
            sort: sort_condition,
        };

        let result = MountainFindCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        let sort_doc = command.options.sort.unwrap();
        assert_eq!(sort_doc.get("elevation").unwrap().as_i64().unwrap(), -1);
    }
}
