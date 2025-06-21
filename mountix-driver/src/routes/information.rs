use crate::model::information::JsonInformationResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn info() -> impl IntoResponse {
    tracing::info!("Access information endpoint.");

    let json: JsonInformationResponse = Default::default();
    (StatusCode::OK, Json(json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_info_endpoint() {
        // Set test environment variables
        std::env::set_var("MOUNTAINS_URL", "https://test.example.com/mountains");
        std::env::set_var("DOCUMENTS_URL", "https://test.example.com/docs");

        let response = info().await;
        let response = response.into_response();

        // Test that information endpoint returns OK status
        assert_eq!(response.status(), StatusCode::OK);

        // Clean up environment variables
        std::env::remove_var("MOUNTAINS_URL");
        std::env::remove_var("DOCUMENTS_URL");
    }
}
