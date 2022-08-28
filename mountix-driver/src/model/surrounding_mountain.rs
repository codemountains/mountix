use mountix_app::model::surrounding_mountain::{
    SearchedSurroundingMountain, SearchedSurroundingMountainLocation,
    SearchedSurroundingMountainResult, SurroundingMountainSearchQuery,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSurroundingMountain {
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: JsonSurroundingMountainLocation,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSurroundingMountainLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

impl From<SearchedSurroundingMountain> for JsonSurroundingMountain {
    fn from(searched_mountain: SearchedSurroundingMountain) -> Self {
        JsonSurroundingMountain {
            id: searched_mountain.id,
            name: searched_mountain.name,
            name_kana: searched_mountain.name_kana,
            area: searched_mountain.area,
            prefectures: searched_mountain.prefectures,
            elevation: searched_mountain.elevation,
            location: searched_mountain.location.into(),
            tags: searched_mountain.tags,
        }
    }
}

impl From<SearchedSurroundingMountainLocation> for JsonSurroundingMountainLocation {
    fn from(searched_location: SearchedSurroundingMountainLocation) -> Self {
        Self {
            latitude: searched_location.latitude,
            longitude: searched_location.longitude,
            gsi_url: searched_location.gsi_url,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSurroundingMountainResponse {
    mountains: Vec<JsonSurroundingMountain>,
    distance: u32,
}

impl From<SearchedSurroundingMountainResult> for JsonSurroundingMountainResponse {
    fn from(result: SearchedSurroundingMountainResult) -> Self {
        let mountains = result
            .mountains
            .into_iter()
            .map(|mountain| mountain.into())
            .collect();

        Self {
            mountains,
            distance: result.distance,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SurroundingMountainQuery {
    distance: Option<String>,
}

impl From<SurroundingMountainQuery> for SurroundingMountainSearchQuery {
    fn from(query: SurroundingMountainQuery) -> Self {
        SurroundingMountainSearchQuery {
            distance: query.distance,
        }
    }
}
