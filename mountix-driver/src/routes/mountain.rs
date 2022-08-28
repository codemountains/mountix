use crate::model::mountain::{
    JsonBoxMountainsResponse, JsonMountain, JsonMountainsResponse, MountainBoxQuery, MountainError,
    MountainQuery,
};
use crate::model::JsonErrorResponse;
use crate::module::{Modules, ModulesExt};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use mountix_app::model::mountain::MountainSearchQuery;
use mountix_kernel::model::ErrorCode;
use std::sync::Arc;
use tracing::log::error;

pub async fn get_mountain(
    Path(mountain_id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, MountainError> {
    let res = modules.mountain_use_case().get(mountain_id).await;
    match res {
        Ok(sm) => {
            return match sm {
                Some(sm) => {
                    tracing::info!("Succeeded to get mountain by id ({}).", &sm.id);

                    let json: JsonMountain = sm.into();
                    Ok((StatusCode::OK, Json(json)))
                }
                None => {
                    tracing::info!("Succeeded to get mountain by id (None).");
                    Err(MountainError::NotFound)
                }
            }
        }
        Err(get_ex) => {
            error!("{:?}", get_ex);
            if get_ex.error_code == ErrorCode::InvalidId {
                Err(MountainError::NotFound)
            } else {
                Err(MountainError::ServerError)
            }
        }
    }
}

pub async fn find_mountains(
    Query(query): Query<MountainQuery>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let search_query: MountainSearchQuery = query.into();

    let res = modules.mountain_use_case().find(search_query).await;
    match res {
        Ok(result) => {
            tracing::info!("Succeeded to find {} mountains.", &result.mountains.len());

            let json: JsonMountainsResponse = result.into();
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

pub async fn find_mountains_by_box(
    Query(query): Query<MountainBoxQuery>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match query.try_into() {
        Ok(search_query) => {
            let res = modules.mountain_use_case().find_box(search_query).await;
            match res {
                Ok(result) => {
                    tracing::info!(
                        "Succeeded to find {} mountains by box.",
                        &result.mountains.len()
                    );

                    let json: JsonBoxMountainsResponse = result.into();
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
        Err(messages) => {
            error!("{:?}", messages);

            let json = JsonErrorResponse::new(messages);
            Err((StatusCode::BAD_REQUEST, Json(json)))
        }
    }
}
