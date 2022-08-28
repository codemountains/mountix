use mongodb::bson::{doc, Document};
use mountix_kernel::model::surrounding_mountain::{
    SurroundingMountain, SurroundingMountainLocation, SurroundingMountainSearchCondition,
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

        let mountain_location = SurroundingMountainLocation::new(
            mountain_doc.location.coordinates[1],
            mountain_doc.location.coordinates[0],
            mountain_doc.gsi_url,
        );

        Ok(SurroundingMountain::new(
            mountain_id,
            mountain_doc.name,
            mountain_doc.name_kana,
            mountain_doc.area,
            mountain_doc.prefectures,
            mountain_doc.elevation,
            mountain_location,
            mountain_doc.tags,
        ))
    }
}

pub struct SurroundingMountainFindCommand {
    pub(crate) filter: Document,
}

impl TryFrom<SurroundingMountainSearchCondition> for SurroundingMountainFindCommand {
    type Error = anyhow::Error;

    fn try_from(sc: SurroundingMountainSearchCondition) -> Result<Self, Self::Error> {
        let coordinates = (
            sc.mountain.location.longitude,
            sc.mountain.location.latitude,
        );
        let filter = doc! {"$and": [{"location":{"$nearSphere": {"$geometry": { "type": "Point",  "coordinates": [coordinates.0, coordinates.1]},"$minDistance": 0,"$maxDistance": sc.distance.0}}}, {"_id": {"$ne": &sc.mountain.id.value}}]};

        Ok(SurroundingMountainFindCommand { filter })
    }
}
