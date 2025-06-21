use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mountix_kernel::model::mountain::{
    Mountain, MountainBoxSearchCondition, MountainData, MountainLocation, MountainSearchCondition,
};
use mountix_kernel::model::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MountainDocument {
    #[serde(rename = "_id")]
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub tags: Vec<String>,
    pub location: MountainLocationDocument,
    pub gsi_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MountainLocationDocument {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

impl TryFrom<MountainDocument> for Mountain {
    type Error = anyhow::Error;
    fn try_from(mountain_doc: MountainDocument) -> Result<Self, Self::Error> {
        let mountain_id: Id<Mountain> = mountain_doc.id.into();

        // MongoDBの地理的データ形式: [longitude, latitude]
        // Rustの座標形式: (latitude, longitude)
        let mountain_location = MountainLocation::new(
            mountain_doc.location.coordinates[1], // latitude (緯度)
            mountain_doc.location.coordinates[0], // longitude (経度)
            mountain_doc.gsi_url,
        );

        let data = MountainData {
            name: mountain_doc.name,
            name_kana: mountain_doc.name_kana,
            area: mountain_doc.area,
            prefectures: mountain_doc.prefectures,
            elevation: mountain_doc.elevation,
            location: mountain_location,
            tags: mountain_doc.tags,
        };
        Ok(Mountain::new(mountain_id, data))
    }
}

pub struct MountainFindCommand {
    pub(crate) filter: Document,
    pub(crate) options: FindOptions,
}

impl TryFrom<MountainSearchCondition> for MountainFindCommand {
    type Error = anyhow::Error;

    fn try_from(sc: MountainSearchCondition) -> Result<Self, Self::Error> {
        let mut filter = Document::new();
        let mut and_doc: Vec<Document> = Vec::new();

        if let Some(name) = sc.name {
            and_doc.push(doc! {"$or": [{"name": {"$regex": &name, "$options": "i"}}, {"name_kana": {"$regex": &name, "$options": "i"}}]});
        }

        if let Some(pref) = sc.prefecture {
            let pref_name = pref.name;
            and_doc.push(doc! {"prefectures": &pref_name});
        }

        if let Some(tag) = sc.tag {
            let tag_name = tag.name;
            and_doc.push(doc! {"tags": &tag_name});
        }

        if !and_doc.is_empty() {
            filter.insert("$and", and_doc);
        }

        let key = sc.sort.key.to_key();
        let value = sc.sort.order.to_value();
        let sort_doc = doc! {key: value};

        let options = FindOptions::builder()
            .sort(sort_doc)
            .skip(sc.skip)
            .limit(sc.limit)
            .build();

        Ok(MountainFindCommand { filter, options })
    }
}

pub struct MountainFindBoxCommand {
    pub(crate) filter: Document,
    pub(crate) options: FindOptions,
}

impl TryFrom<MountainBoxSearchCondition> for MountainFindBoxCommand {
    type Error = anyhow::Error;

    fn try_from(sc: MountainBoxSearchCondition) -> Result<Self, Self::Error> {
        let mut filter = Document::new();
        // MongoDBの地理的ボックス検索では [longitude, latitude] の順序が必要
        let mut and_doc = vec![
            doc! {"location": {"$geoWithin": {"$box": [[sc.box_coordinates.bottom_left.0,sc.box_coordinates.bottom_left.1], [sc.box_coordinates.upper_right.0,sc.box_coordinates.upper_right.1]]}}},
        ];

        if let Some(name) = sc.name {
            and_doc.push(doc! {"$or": [{"name": {"$regex": &name, "$options": "i"}}, {"name_kana": {"$regex": &name, "$options": "i"}}]});
        }

        if let Some(tag) = sc.tag {
            let tag_name = tag.name;
            and_doc.push(doc! {"tags": &tag_name});
        }

        filter.insert("$and", and_doc);

        let key = sc.sort.key.to_key();
        let value = sc.sort.order.to_value();
        let sort_doc = doc! {key: value};

        let options = FindOptions::builder().sort(sort_doc).build();

        Ok(MountainFindBoxCommand { filter, options })
    }
}
