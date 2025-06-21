use mongodb::bson::{doc, Document};
use mountix_kernel::model::surrounding_mountain::{
    SurroundingMountain, SurroundingMountainData, SurroundingMountainLocation,
    SurroundingMountainSearchCondition,
};
use mountix_kernel::model::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SurroundingMountainDocument {
    #[serde(rename = "_id")]
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub tags: Vec<String>,
    pub location: SurroundingMountainLocationDocument,
    pub gsi_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurroundingMountainLocationDocument {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

impl TryFrom<SurroundingMountainDocument> for SurroundingMountain {
    type Error = anyhow::Error;
    fn try_from(mountain_doc: SurroundingMountainDocument) -> Result<Self, Self::Error> {
        let mountain_id: Id<SurroundingMountain> = mountain_doc.id.into();

        // MongoDBの地理的データ形式: [longitude, latitude]
        // Rustの座標形式: (latitude, longitude)
        let mountain_location = SurroundingMountainLocation::new(
            mountain_doc.location.coordinates[1], // latitude (緯度)
            mountain_doc.location.coordinates[0], // longitude (経度)
            mountain_doc.gsi_url,
        );

        let data = SurroundingMountainData {
            name: mountain_doc.name,
            name_kana: mountain_doc.name_kana,
            area: mountain_doc.area,
            prefectures: mountain_doc.prefectures,
            elevation: mountain_doc.elevation,
            location: mountain_location,
            tags: mountain_doc.tags,
        };
        Ok(SurroundingMountain::new(mountain_id, data))
    }
}

pub struct SurroundingMountainFindCommand {
    pub(crate) filter: Document,
}

impl TryFrom<SurroundingMountainSearchCondition> for SurroundingMountainFindCommand {
    type Error = anyhow::Error;

    fn try_from(sc: SurroundingMountainSearchCondition) -> Result<Self, Self::Error> {
        // MongoDBの地理的クエリでは [longitude, latitude] の順序が必要
        let coordinates = (
            sc.mountain.location.longitude, // longitude (経度)
            sc.mountain.location.latitude,  // latitude (緯度)
        );
        let filter = doc! {"$and": [{"location":{"$nearSphere": {"$geometry": { "type": "Point",  "coordinates": [coordinates.0, coordinates.1]},"$minDistance": 0,"$maxDistance": sc.distance.0}}}, {"_id": {"$ne": &sc.mountain.id.value}}]};

        Ok(SurroundingMountainFindCommand { filter })
    }
}
