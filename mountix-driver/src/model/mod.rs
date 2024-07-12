use serde::Serialize;

pub mod information;
pub mod mountain;
pub mod surrounding_mountain;

/// Error response struct
///
/// エラー応答レスポンス
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorResponse {
    messages: Vec<String>,
}

impl JsonErrorResponse {
    /// Returns error response
    ///
    /// エラー応答レスポンスを生成します
    pub(crate) fn new(messages: Vec<String>) -> Self {
        Self { messages }
    }
}
