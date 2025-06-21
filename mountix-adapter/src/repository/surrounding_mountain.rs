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
        let mut mountain_doc_list = collection.find(find_command.filter).await?;

        let mut mountains: Vec<SurroundingMountain> = Vec::new();
        while let Some(sd) = mountain_doc_list.try_next().await? {
            mountains.push(sd.try_into()?);
        }

        Ok(mountains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::surrounding_mountain::{
        SurroundingMountainDocument, SurroundingMountainLocationDocument,
    };
    use mountix_kernel::model::mountain::{Mountain, MountainData, MountainLocation};
    use mountix_kernel::model::surrounding_mountain::{
        SurroundingMountainSearchCondition, SurroundingMountainSearchDistance,
    };
    use mountix_kernel::model::Id;

    fn create_test_surrounding_mountain_document() -> SurroundingMountainDocument {
        SurroundingMountainDocument {
            id: 2,
            name: "周辺の山".to_string(),
            name_kana: "しゅうへんのやま".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string()],
            elevation: 2500,
            tags: vec!["二百名山".to_string()],
            location: SurroundingMountainLocationDocument {
                r#type: "Point".to_string(),
                coordinates: [138.800000, 35.300000], // [longitude, latitude] - MongoDB形式
            },
            gsi_url: "https://maps.gsi.go.jp/surrounding".to_string(),
        }
    }

    fn create_test_mountain() -> Mountain {
        let id = Id::new(1);
        let location =
            MountainLocation::new(35.360556, 138.727778, "https://maps.gsi.go.jp".to_string());
        let data = MountainData {
            name: "富士山".to_string(),
            name_kana: "ふじさん".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string(), "山梨県".to_string()],
            elevation: 3776,
            location,
            tags: vec!["百名山".to_string()],
        };
        Mountain::new(id, data)
    }

    #[test]
    fn test_surrounding_mountain_document_to_surrounding_mountain_conversion() {
        let surrounding_mountain_doc = create_test_surrounding_mountain_document();
        let result = SurroundingMountain::try_from(surrounding_mountain_doc);

        assert!(result.is_ok());
        let surrounding_mountain = result.unwrap();
        assert_eq!(surrounding_mountain.id.value, 2);
        assert_eq!(surrounding_mountain.name, "周辺の山");
        assert_eq!(surrounding_mountain.name_kana, "しゅうへんのやま");
        assert_eq!(surrounding_mountain.area, "関東地方");
        assert_eq!(surrounding_mountain.prefectures, vec!["静岡県"]);
        assert_eq!(surrounding_mountain.elevation, 2500);
        assert_eq!(surrounding_mountain.location.latitude, 35.300000);
        assert_eq!(surrounding_mountain.location.longitude, 138.800000);
        assert_eq!(
            surrounding_mountain.location.gsi_url,
            "https://maps.gsi.go.jp/surrounding"
        );
        assert_eq!(surrounding_mountain.tags, vec!["二百名山"]);
    }

    #[test]
    fn test_surrounding_mountain_search_condition_to_find_command() {
        let mountain = create_test_mountain();
        let distance = SurroundingMountainSearchDistance::new(10000);
        let search_condition = SurroundingMountainSearchCondition::new(mountain, distance);

        let result = SurroundingMountainFindCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        // Test that the filter contains geospatial query structure
        assert!(command.filter.contains_key("$and"));
    }

    #[test]
    fn test_surrounding_mountain_search_condition_with_default_distance() {
        let mountain = create_test_mountain();
        let distance = SurroundingMountainSearchDistance::default();
        let search_condition = SurroundingMountainSearchCondition::new(mountain, distance);

        let result = SurroundingMountainFindCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        // Test that the filter contains geospatial query structure with default distance
        assert!(command.filter.contains_key("$and"));
    }

    #[test]
    fn test_surrounding_mountain_search_condition_with_large_distance() {
        let mountain = create_test_mountain();
        let distance = SurroundingMountainSearchDistance::new(50000); // 50km
        let search_condition = SurroundingMountainSearchCondition::new(mountain, distance);

        let result = SurroundingMountainFindCommand::try_from(search_condition);
        assert!(result.is_ok());

        let command = result.unwrap();
        // Test that the filter contains geospatial query structure with large distance
        assert!(command.filter.contains_key("$and"));
    }

    #[test]
    fn test_surrounding_mountain_location_document_structure() {
        let location_doc = SurroundingMountainLocationDocument {
            r#type: "Point".to_string(),
            coordinates: [138.727778, 35.360556], // [longitude, latitude] - MongoDB形式
        };

        assert_eq!(location_doc.r#type, "Point");
        assert_eq!(location_doc.coordinates[0], 138.727778); // longitude (経度)
        assert_eq!(location_doc.coordinates[1], 35.360556); // latitude (緯度)
    }

    #[test]
    fn test_surrounding_mountain_document_structure() {
        let doc = create_test_surrounding_mountain_document();

        assert_eq!(doc.id, 2);
        assert_eq!(doc.name, "周辺の山");
        assert_eq!(doc.elevation, 2500);
        assert_eq!(doc.location.r#type, "Point");
        assert_eq!(doc.location.coordinates[0], 138.800000); // longitude (経度)
        assert_eq!(doc.location.coordinates[1], 35.300000); // latitude (緯度)
    }
}
