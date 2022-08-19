use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use mountix_app::model::mountain::{
    FoundMountains, MountainSearchQuery, SearchedLocation, SearchedMountain,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMountain {
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: JsonLocation,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

impl From<SearchedMountain> for JsonMountain {
    fn from(searched_mountain: SearchedMountain) -> Self {
        JsonMountain {
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

impl From<SearchedLocation> for JsonLocation {
    fn from(searched_location: SearchedLocation) -> Self {
        Self {
            latitude: searched_location.latitude,
            longitude: searched_location.longitude,
            gsi_url: searched_location.gsi_url,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMountainsResponse {
    mountains: Vec<JsonMountain>,
    total: u64,
    offset: u64,
    limit: Option<u64>,
}

impl From<FoundMountains> for JsonMountainsResponse {
    fn from(fm: FoundMountains) -> Self {
        let mut mountains: Vec<JsonMountain> = Vec::new();
        for sm in fm.mountains {
            mountains.push(sm.into());
        }

        Self {
            mountains,
            total: fm.total,
            offset: fm.offset,
            limit: fm.limit,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MountainQuery {
    name: Option<String>,
    prefecture: Option<String>,
    tag: Option<String>,
    offset: Option<String>,
    limit: Option<String>,
    sort: Option<String>,
}

impl From<MountainQuery> for MountainSearchQuery {
    fn from(mq: MountainQuery) -> Self {
        MountainSearchQuery {
            name: mq.name,
            prefecture: mq.prefecture,
            tag: mq.tag,
            offset: mq.offset,
            limit: mq.limit,
            sort: mq.sort,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMountainsErrorResponse {
    messages: Vec<String>,
}

impl JsonMountainsErrorResponse {
    pub(crate) fn new(messages: Vec<String>) -> Self {
        Self { messages }
    }
}

pub enum MountainError {
    NotFound,
    ServerError,
}

impl IntoResponse for MountainError {
    fn into_response(self) -> Response {
        match self {
            MountainError::NotFound => {
                let json = JsonMountainsErrorResponse::new(vec![
                    "山岳情報が見つかりませんでした。".to_string(),
                ]);
                (StatusCode::NOT_FOUND, Json(json)).into_response()
            }
            MountainError::ServerError => {
                let json = JsonMountainsErrorResponse::new(vec![
                    "山岳情報を検索中にエラーが発生しました。".to_string(),
                ]);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
            }
        }
    }
}
