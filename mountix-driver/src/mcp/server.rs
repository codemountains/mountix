use crate::model::mountain::{JsonBoxMountainsResponse, JsonMountain, JsonMountainsResponse};
use crate::model::surrounding_mountain::JsonSurroundingMountainResponse;
use crate::module::{Modules, ModulesExt};
use mountix_app::model::mountain::{MountainBoxSearchQuery, MountainSearchQuery};
use mountix_app::model::surrounding_mountain::SurroundingMountainSearchQuery;
use mountix_kernel::model::ErrorCode;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct MountixMcpServer {
    modules: Arc<Modules>,
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl MountixMcpServer {
    pub fn new(modules: Arc<Modules>) -> Self {
        Self {
            modules,
            tool_router: Self::tool_router(),
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetMountainParams {
    #[schemars(description = "山岳ID")]
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindMountainsParams {
    #[schemars(description = "山岳名（部分一致）")]
    pub name: Option<String>,
    #[schemars(description = "都道府県ID")]
    pub prefecture: Option<String>,
    #[schemars(description = "タグID")]
    pub tag: Option<String>,
    #[schemars(description = "取得開始位置")]
    pub offset: Option<String>,
    #[schemars(description = "取得件数")]
    pub limit: Option<String>,
    #[schemars(description = "ソート条件")]
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindMountainsByBoxParams {
    #[schemars(description = "検索範囲。形式: (左下経度,左下緯度),(右上経度,右上緯度)")]
    pub box_coordinates: String,
    #[schemars(description = "山岳名（部分一致）")]
    pub name: Option<String>,
    #[schemars(description = "タグID")]
    pub tag: Option<String>,
    #[schemars(description = "ソート条件")]
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindSurroundingsParams {
    #[schemars(description = "基準となる山岳ID")]
    pub mountain_id: String,
    #[schemars(description = "検索距離（メートル）")]
    pub distance: Option<String>,
}

#[tool_router]
impl MountixMcpServer {
    #[tool(description = "IDを指定して山岳情報を1件取得します。")]
    async fn get_mountain(
        &self,
        Parameters(params): Parameters<GetMountainParams>,
    ) -> Result<String, ErrorData> {
        let result = self.modules.mountain_use_case().get(params.id).await;

        match result {
            Ok(Some(mountain)) => {
                let json: JsonMountain = mountain.into();
                to_json_string(&json)
            }
            Ok(None) => Err(ErrorData::invalid_params(
                "山岳情報が見つかりませんでした。",
                None,
            )),
            Err(error) if error.error_code == ErrorCode::InvalidId => Err(
                ErrorData::invalid_params("指定された山岳IDが不正です。", None),
            ),
            Err(_) => Err(ErrorData::internal_error(
                "山岳情報を取得中に予期せぬエラーが発生しました。",
                None,
            )),
        }
    }

    #[tool(description = "条件を指定して山岳情報を検索します。")]
    async fn find_mountains(
        &self,
        Parameters(params): Parameters<FindMountainsParams>,
    ) -> Result<String, ErrorData> {
        let search_query = MountainSearchQuery {
            name: params.name,
            prefecture: params.prefecture,
            tag: params.tag,
            offset: params.offset,
            limit: params.limit,
            sort: params.sort,
        };

        match self.modules.mountain_use_case().find(search_query).await {
            Ok(result) => {
                let json: JsonMountainsResponse = result.into();
                to_json_string(&json)
            }
            Err(error) if error.error_code == ErrorCode::InvalidQueryParam => {
                Err(ErrorData::invalid_params(error.messages.join("\n"), None))
            }
            Err(_) => Err(ErrorData::internal_error(
                "山岳情報を検索中に予期せぬエラーが発生しました。",
                None,
            )),
        }
    }

    #[tool(description = "地理的な矩形範囲内の山岳情報を検索します。")]
    async fn find_mountains_by_box(
        &self,
        Parameters(params): Parameters<FindMountainsByBoxParams>,
    ) -> Result<String, ErrorData> {
        let search_query = MountainBoxSearchQuery {
            box_coordinates: params.box_coordinates,
            name: params.name,
            tag: params.tag,
            sort: params.sort,
        };

        match self
            .modules
            .mountain_use_case()
            .find_box(search_query)
            .await
        {
            Ok(result) => {
                let json: JsonBoxMountainsResponse = result.into();
                to_json_string(&json)
            }
            Err(error) if error.error_code == ErrorCode::InvalidQueryParam => {
                Err(ErrorData::invalid_params(error.messages.join("\n"), None))
            }
            Err(_) => Err(ErrorData::internal_error(
                "山岳情報を範囲検索中に予期せぬエラーが発生しました。",
                None,
            )),
        }
    }

    #[tool(description = "指定した山岳の周辺にある山岳情報を検索します。")]
    async fn find_surroundings(
        &self,
        Parameters(params): Parameters<FindSurroundingsParams>,
    ) -> Result<String, ErrorData> {
        let search_query = SurroundingMountainSearchQuery {
            distance: params.distance,
        };

        match self
            .modules
            .surrounding_mountain_use_case()
            .find(params.mountain_id, search_query)
            .await
        {
            Ok(result) => {
                let json: JsonSurroundingMountainResponse = result.into();
                to_json_string(&json)
            }
            Err(error) if error.error_code == ErrorCode::InvalidQueryParam => {
                Err(ErrorData::invalid_params(error.messages.join("\n"), None))
            }
            Err(_) => Err(ErrorData::internal_error(
                "周辺の山岳情報を検索中に予期せぬエラーが発生しました。",
                None,
            )),
        }
    }
}

#[tool_handler(
    name = "mountix",
    instructions = "Mountix 日本の山岳データ API。百名山などの山岳情報を検索・取得できます。"
)]
impl ServerHandler for MountixMcpServer {}

fn to_json_string<T: serde::Serialize>(value: &T) -> Result<String, ErrorData> {
    serde_json::to_string_pretty(value).map_err(|error| {
        ErrorData::internal_error(format!("JSONの生成に失敗しました: {error}"), None)
    })
}
