use serde::Serialize;
use std::env;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonInformationResponse {
    about: String,
    endpoints: Vec<JsonEndpoint>,
    documents: String,
}

impl Default for JsonInformationResponse {
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonEndpoint {
    resource: String,
    url: String,
}

impl JsonEndpoint {
    fn new(resource: String, url: String) -> Self {
        Self { resource, url }
    }
}
