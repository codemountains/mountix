use crate::model::JsonErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use mountix_app::model::mountain::{
    MountainBoxSearchQuery, MountainSearchQuery, SearchedBoxMountainResult, SearchedMountain,
    SearchedMountainLocation, SearchedMountainResult,
};
use serde::{Deserialize, Serialize};

/// Mountain json object
///
/// 山岳情報
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMountain {
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: JsonMountainLocation,
    pub tags: Vec<String>,
}

/// Mountain location json object
///
/// 山岳位置情報
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMountainLocation {
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

impl From<SearchedMountainLocation> for JsonMountainLocation {
    fn from(searched_location: SearchedMountainLocation) -> Self {
        Self {
            latitude: searched_location.latitude,
            longitude: searched_location.longitude,
            gsi_url: searched_location.gsi_url,
        }
    }
}

/// Mountain response
///
/// 山岳情報レスポンス
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMountainsResponse {
    mountains: Vec<JsonMountain>,
    total: u64,
    offset: u64,
    limit: Option<u64>,
}

impl From<SearchedMountainResult> for JsonMountainsResponse {
    /// Converts to `JsonMountainResponse` from `SearchedMountainResult`
    ///
    /// 山岳情報検索結果から山岳情報レスポンスに変換します
    fn from(result: SearchedMountainResult) -> Self {
        let mountains = result
            .mountains
            .into_iter()
            .map(|mountain| mountain.into())
            .collect();

        Self {
            mountains,
            total: result.total,
            offset: result.offset,
            limit: result.limit,
        }
    }
}

/// Mountain search query object
///
/// 山岳情報検索クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct MountainSearchQueryParam {
    name: Option<String>,
    prefecture: Option<String>,
    tag: Option<String>,
    offset: Option<String>,
    limit: Option<String>,
    sort: Option<String>,
}

impl From<MountainSearchQueryParam> for MountainSearchQuery {
    /// Converts to `MountainSearchQuery` from `MountainSearchQueryParam`
    ///
    /// 山岳情報検索クエリパラメータから山岳情報検索クエリオブジェクトに変換します
    fn from(mq: MountainSearchQueryParam) -> Self {
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

/// Box mountains response
///
/// 山岳情報範囲検索レスポンス
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonBoxMountainsResponse {
    mountains: Vec<JsonMountain>,
    total: u64,
}

impl From<SearchedBoxMountainResult> for JsonBoxMountainsResponse {
    /// Converts to `JsonBoxMountainsResponse` from `SearchedBoxMountainResult`
    ///
    /// 山岳情報の範囲検索結果から山岳情報範囲検索レスポンスに変換します
    fn from(result: SearchedBoxMountainResult) -> Self {
        let mountains = result
            .mountains
            .into_iter()
            .map(|mountain| mountain.into())
            .collect();

        Self {
            mountains,
            total: result.total,
        }
    }
}

/// Box mountains search query object
///
/// 山岳情報の範囲検索クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct MountainBoxSearchQueryParam {
    r#box: Option<String>,
    name: Option<String>,
    tag: Option<String>,
    sort: Option<String>,
}

impl TryFrom<MountainBoxSearchQueryParam> for MountainBoxSearchQuery {
    type Error = Vec<String>;

    /// Converts to `MountainBoxSearchQuery` from `MountainBoxSearchQueryParam`
    ///
    /// 山岳情報の範囲検索クエリパラメータから山岳情報の範囲検索クエリオブジェクトに変換します
    fn try_from(bq: MountainBoxSearchQueryParam) -> Result<Self, Self::Error> {
        match bq.r#box {
            Some(box_param) => Ok(MountainBoxSearchQuery {
                box_coordinates: box_param,
                name: bq.name,
                tag: bq.tag,
                sort: bq.sort,
            }),
            None => Err(vec!["クエリパラメータ box=(bottom left longitude,bottom left latitude),(upper right longitude,upper right latitude) は必須です。".to_string()]),
        }
    }
}

/// Mountain search error
///
/// 山岳情報検索エラー
pub enum MountainError {
    NotFound,
    ServerError,
}

impl IntoResponse for MountainError {
    /// Create error response
    ///
    /// 山岳情報検索エラー時のレスポンスを生成します
    fn into_response(self) -> Response {
        match self {
            MountainError::NotFound => {
                let json =
                    JsonErrorResponse::new(vec!["山岳情報が見つかりませんでした。".to_string()]);
                (StatusCode::NOT_FOUND, Json(json)).into_response()
            }
            MountainError::ServerError => {
                let json = JsonErrorResponse::new(vec![
                    "山岳情報を取得中に予期せぬエラーが発生しました。".to_string(),
                ]);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
            }
        }
    }
}
