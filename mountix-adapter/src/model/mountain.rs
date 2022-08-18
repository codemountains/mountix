use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mountix_kernel::model::mountain::{Location, Mountain, MountainSearchCondition};
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
    pub location: LocationDocument,
    pub gsi_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationDocument {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

impl TryFrom<MountainDocument> for Mountain {
    type Error = anyhow::Error;
    fn try_from(mountain_doc: MountainDocument) -> Result<Self, Self::Error> {
        let mountain_location = Location::new(
            mountain_doc.location.coordinates[1],
            mountain_doc.location.coordinates[0],
            mountain_doc.gsi_url,
        );

        Ok(Mountain::new(
            mountain_doc.id,
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

        if and_doc.len() > 0 {
            filter.insert("$and", and_doc);
        }

        let key = sc.sort.key.to_key();
        let value = sc.sort.order.to_type();
        let sort_doc = doc! {key: value};

        let options = FindOptions::builder()
            .sort(sort_doc)
            .skip(sc.skip)
            .limit(sc.limit)
            .build();

        Ok(MountainFindCommand { filter, options })
    }
}
