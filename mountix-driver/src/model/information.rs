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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_endpoint_new() {
        let resource = "mountains".to_string();
        let url = "https://api.example.com/mountains".to_string();
        let endpoint = JsonEndpoint::new(resource.clone(), url.clone());

        assert_eq!(endpoint.resource, resource);
        assert_eq!(endpoint.url, url);
    }

    #[test]
    fn test_json_endpoint_structure() {
        let endpoint = JsonEndpoint {
            resource: "test".to_string(),
            url: "https://test.com".to_string(),
        };

        assert_eq!(endpoint.resource, "test");
        assert_eq!(endpoint.url, "https://test.com");
    }

    #[test]
    fn test_json_information_response_with_env_vars() {
        // Set test environment variables
        std::env::set_var("MOUNTAINS_URL", "https://test.example.com/mountains");
        std::env::set_var("DOCUMENTS_URL", "https://test.example.com/docs");

        let response = JsonInformationResponse::default();

        assert_eq!(
            response.about,
            "日本の主な山岳をJSON形式で提供するAPIです。"
        );
        assert_eq!(response.endpoints.len(), 1);
        assert_eq!(response.endpoints[0].resource, "mountains");
        assert_eq!(
            response.endpoints[0].url,
            "https://test.example.com/mountains"
        );
        assert_eq!(response.documents, "https://test.example.com/docs");

        // Clean up environment variables
        std::env::remove_var("MOUNTAINS_URL");
        std::env::remove_var("DOCUMENTS_URL");
    }

    #[test]
    fn test_json_endpoint_multiple_resources() {
        let endpoints = vec![
            JsonEndpoint::new(
                "mountains".to_string(),
                "https://api.com/mountains".to_string(),
            ),
            JsonEndpoint::new("health".to_string(), "https://api.com/health".to_string()),
        ];

        assert_eq!(endpoints.len(), 2);
        assert_eq!(endpoints[0].resource, "mountains");
        assert_eq!(endpoints[1].resource, "health");
    }

    #[test]
    fn test_json_information_response_structure() {
        // Test structure without environment dependencies
        std::env::set_var("MOUNTAINS_URL", "https://example.com/mountains");
        std::env::set_var("DOCUMENTS_URL", "https://example.com/docs");

        let response = JsonInformationResponse::default();

        // Test that all required fields are present
        assert!(!response.about.is_empty());
        assert!(!response.endpoints.is_empty());
        assert!(!response.documents.is_empty());

        std::env::remove_var("MOUNTAINS_URL");
        std::env::remove_var("DOCUMENTS_URL");
    }
}
