use crate::module::{Modules, ModulesExt};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;
use tracing::error;

pub async fn hc() -> impl IntoResponse {
    tracing::debug!("Access health check endpoint.");
    StatusCode::NO_CONTENT
}

pub async fn hc_mongodb(
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .health_check_use_case()
        .diagnose_mongo_db_conn()
        .await
        .map(|_| {
            tracing::debug!("Access mongodb health check endpoint.");
            StatusCode::NO_CONTENT
        })
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::SERVICE_UNAVAILABLE
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hc_endpoint() {
        let response = hc().await;

        // Test that health check returns NO_CONTENT status
        assert_eq!(response.into_response().status(), StatusCode::NO_CONTENT);
    }

    // Note: MongoDB health check tests would require actual database connection
    // The hc_mongodb function tests are omitted as they need real infrastructure
}
