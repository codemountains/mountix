use crate::model::surrounding_mountain::{
    JsonSurroundingMountainResponse, SurroundingMountainSearchQueryParam,
};
use crate::model::JsonErrorResponse;
use crate::module::{Modules, ModulesExt};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use mountix_app::model::surrounding_mountain::SurroundingMountainSearchQuery;
use mountix_kernel::model::ErrorCode;
use std::sync::Arc;
use tracing::log::error;

pub async fn find_surroundings(
    Path(mountain_id): Path<String>,
    Query(query): Query<SurroundingMountainSearchQueryParam>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let search_query: SurroundingMountainSearchQuery = query.into();

    let res = modules
        .surrounding_mountain_use_case()
        .find(mountain_id, search_query)
        .await;
    match res {
        Ok(result) => {
            tracing::info!(
                "Succeeded to find {} surroundings.",
                &result.mountains.len()
            );

            let json: JsonSurroundingMountainResponse = result.into();
            Ok((StatusCode::OK, Json(json)))
        }
        Err(find_ex) => {
            error!("{:?}", find_ex);

            let json = JsonErrorResponse::new(find_ex.messages);
            if find_ex.error_code == ErrorCode::ServerError {
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json)))
            } else {
                Err((StatusCode::BAD_REQUEST, Json(json)))
            }
        }
    }
}
