use serde::Serialize;
use std::env;

/// API Info response
///
/// API 情報応答レスポンス
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonInformationResponse {
    about: String,
    endpoints: Vec<JsonEndpoint>,
    documents: String,
}

impl Default for JsonInformationResponse {
    /// Return API Info response
    ///
    /// API 情報応答レスポンスを生成します
    fn default() -> Self {
        let mountains_url = env::var("MOUNTAINS_URL").expect("MOUNTAINS_URL is undefined.");
        let documents_url = env::var("DOCUMENTS_URL").expect("DOCUMENTS_URL is undefined.");

        let about = "日本の主な山岳をJSON形式で提供するAPIです。".to_string();
        let endpoints = vec![JsonEndpoint::new("mountains".to_string(), mountains_url)];
        let documents = documents_url;

        Self {
            about,
            endpoints,
            documents,
        }
    }
}

/// API endpoint struct
///
/// API エンドポイント情報
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonEndpoint {
    resource: String,
    url: String,
}

impl JsonEndpoint {
    /// Returns API endpoint struct
    ///
    /// API エンドポイント情報を生成します
    fn new(resource: String, url: String) -> Self {
        Self { resource, url }
    }
}
