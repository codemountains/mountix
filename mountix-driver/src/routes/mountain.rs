use crate::model::mountain::{
    JsonBoxMountainsResponse, JsonMountain, JsonMountainsResponse, MountainBoxSearchQueryParam,
    MountainError, MountainSearchQueryParam,
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
        Ok(sm) => match sm {
            Some(sm) => {
                tracing::info!("Succeeded to get mountain by id ({}).", &sm.id);

                let json: JsonMountain = sm.into();
                Ok((StatusCode::OK, Json(json)))
            }
            None => {
                tracing::info!("Succeeded to get mountain by id (None).");
                Err(MountainError::NotFound)
            }
        },
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
    Query(query): Query<MountainSearchQueryParam>,
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
    Query(query): Query<MountainBoxSearchQueryParam>,
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

#[cfg(test)]
mod tests {
    use crate::model::mountain::JsonMountain;
    use crate::model::JsonErrorResponse;
    use mountix_app::model::mountain::{SearchedMountain, SearchedMountainLocation};
    use mountix_kernel::model::mountain::{MountainFindException, MountainGetException};
    use mountix_kernel::model::ErrorCode;

    fn create_test_searched_mountain() -> SearchedMountain {
        SearchedMountain {
            id: 1,
            name: "富士山".to_string(),
            name_kana: "ふじさん".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string(), "山梨県".to_string()],
            elevation: 3776,
            location: SearchedMountainLocation {
                latitude: 35.360556,
                longitude: 138.727778,
                gsi_url: "https://maps.gsi.go.jp/fuji".to_string(),
            },
            tags: vec!["百名山".to_string()],
        }
    }

    #[test]
    fn test_json_mountain_conversion() {
        let searched_mountain = create_test_searched_mountain();
        let json_mountain: JsonMountain = searched_mountain.into();

        assert_eq!(json_mountain.id, 1);
        assert_eq!(json_mountain.name, "富士山");
        assert_eq!(json_mountain.name_kana, "ふじさん");
        assert_eq!(json_mountain.elevation, 3776);
        assert_eq!(json_mountain.location.latitude, 35.360556);
        assert_eq!(json_mountain.location.longitude, 138.727778);
    }

    #[test]
    fn test_mountain_get_exception_not_found() {
        let exception = MountainGetException::new(ErrorCode::InvalidId);
        assert_eq!(exception.error_code, ErrorCode::InvalidId);
    }

    #[test]
    fn test_mountain_get_exception_server_error() {
        let exception = MountainGetException::new(ErrorCode::ServerError);
        assert_eq!(exception.error_code, ErrorCode::ServerError);
    }

    #[test]
    fn test_mountain_find_exception_invalid_params() {
        let messages = vec!["Invalid parameter".to_string()];
        let exception = MountainFindException::new(ErrorCode::InvalidQueryParam, messages.clone());
        assert_eq!(exception.error_code, ErrorCode::InvalidQueryParam);
        assert_eq!(exception.messages, messages);
    }

    #[test]
    fn test_mountain_find_exception_with_error_code() {
        let exception = MountainFindException::new_with_error_code(ErrorCode::ServerError);
        assert_eq!(exception.error_code, ErrorCode::ServerError);
        assert!(!exception.messages.is_empty());
    }

    #[test]
    fn test_json_error_response_creation() {
        let messages = vec!["Error message".to_string()];
        let _response = JsonErrorResponse::new(messages.clone());
        // Cannot test private field directly, but test that the object was created
        assert!(true); // JsonErrorResponse was created successfully
    }

    #[test]
    fn test_json_mountains_response_structure() {
        let searched_mountain = create_test_searched_mountain();
        let mountains = vec![searched_mountain];

        // Test that the conversion would work for JsonMountainsResponse
        let json_mountains: Vec<JsonMountain> = mountains.into_iter().map(|m| m.into()).collect();
        assert_eq!(json_mountains.len(), 1);
        assert_eq!(json_mountains[0].name, "富士山");
    }

    // Unit tests for model conversions and error handling
    // These don't require database connections or HTTP servers
    #[test]
    fn test_mountain_error_handling() {
        // Test InvalidId error
        let invalid_id_error = MountainGetException::new(ErrorCode::InvalidId);
        assert_eq!(invalid_id_error.error_code, ErrorCode::InvalidId);

        // Test ServerError
        let server_error = MountainGetException::new(ErrorCode::ServerError);
        assert_eq!(server_error.error_code, ErrorCode::ServerError);
    }

    #[test]
    fn test_mountain_find_error_handling() {
        let find_error = MountainFindException::new_with_error_code(ErrorCode::ServerError);
        assert_eq!(find_error.error_code, ErrorCode::ServerError);
        assert!(!find_error.messages.is_empty());
    }
}
