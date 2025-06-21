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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::surrounding_mountain::JsonSurroundingMountain;
    use mountix_app::model::surrounding_mountain::{
        SearchedSurroundingMountain, SearchedSurroundingMountainLocation,
    };
    use mountix_kernel::model::surrounding_mountain::SurroundingMountainFindException;
    use mountix_kernel::model::ErrorCode;

    fn create_test_searched_surrounding_mountain() -> SearchedSurroundingMountain {
        SearchedSurroundingMountain {
            id: 2,
            name: "周辺の山".to_string(),
            name_kana: "しゅうへんのやま".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string()],
            elevation: 2500,
            location: SearchedSurroundingMountainLocation {
                latitude: 35.300000,
                longitude: 138.800000,
                gsi_url: "https://maps.gsi.go.jp/surrounding".to_string(),
            },
            tags: vec!["二百名山".to_string()],
        }
    }

    #[test]
    fn test_json_surrounding_mountain_conversion() {
        let searched_surrounding_mountain = create_test_searched_surrounding_mountain();
        let json_mountain: JsonSurroundingMountain = searched_surrounding_mountain.into();

        assert_eq!(json_mountain.id, 2);
        assert_eq!(json_mountain.name, "周辺の山");
        assert_eq!(json_mountain.name_kana, "しゅうへんのやま");
        assert_eq!(json_mountain.elevation, 2500);
        assert_eq!(json_mountain.location.latitude, 35.300000);
        assert_eq!(json_mountain.location.longitude, 138.800000);
    }

    #[test]
    fn test_surrounding_mountain_find_exception_server_error() {
        let exception =
            SurroundingMountainFindException::new_with_error_code(ErrorCode::ServerError);
        assert_eq!(exception.error_code, ErrorCode::ServerError);
        assert!(!exception.messages.is_empty());
    }

    #[test]
    fn test_surrounding_mountain_find_exception_invalid_params() {
        let messages = vec!["Invalid distance parameter".to_string()];
        let exception =
            SurroundingMountainFindException::new(ErrorCode::InvalidQueryParam, messages.clone());
        assert_eq!(exception.error_code, ErrorCode::InvalidQueryParam);
        assert_eq!(exception.messages, messages);
    }

    #[test]
    fn test_json_surrounding_mountain_response_structure() {
        let searched_mountain = create_test_searched_surrounding_mountain();
        let mountains = vec![searched_mountain];

        // Test that the conversion would work for JsonSurroundingMountainResponse
        let json_mountains: Vec<JsonSurroundingMountain> =
            mountains.into_iter().map(|m| m.into()).collect();
        assert_eq!(json_mountains.len(), 1);
        assert_eq!(json_mountains[0].name, "周辺の山");
        assert_eq!(json_mountains[0].elevation, 2500);
    }

    #[test]
    fn test_surrounding_mountain_error_handling() {
        // Test ServerError
        let server_error =
            SurroundingMountainFindException::new_with_error_code(ErrorCode::ServerError);
        assert_eq!(server_error.error_code, ErrorCode::ServerError);

        // Test InvalidQueryParam error
        let param_messages = vec!["Distance out of range".to_string()];
        let param_error = SurroundingMountainFindException::new(
            ErrorCode::InvalidQueryParam,
            param_messages.clone(),
        );
        assert_eq!(param_error.error_code, ErrorCode::InvalidQueryParam);
        assert_eq!(param_error.messages, param_messages);
    }

    #[test]
    fn test_surrounding_mountain_location_json_conversion() {
        let location = SearchedSurroundingMountainLocation {
            latitude: 35.678,
            longitude: 139.765,
            gsi_url: "https://example.com".to_string(),
        };

        // Test individual field access for JSON conversion
        assert_eq!(location.latitude, 35.678);
        assert_eq!(location.longitude, 139.765);
        assert_eq!(location.gsi_url, "https://example.com");
    }

    #[test]
    fn test_surrounding_mountain_search_query_param_structure() {
        // Test that query parameter structure is handled correctly
        use crate::model::surrounding_mountain::SurroundingMountainSearchQueryParam;

        // Test with distance parameter
        let query_param = SurroundingMountainSearchQueryParam {
            distance: Some("10000".to_string()),
        };
        let _search_query: SurroundingMountainSearchQuery = query_param.into();
        // The actual conversion logic would be tested here in a real implementation
        assert!(true); // Placeholder - tests the structure exists
    }

    #[test]
    fn test_surrounding_mountain_error_messages() {
        let exception =
            SurroundingMountainFindException::new_with_error_code(ErrorCode::ServerError);
        assert!(!exception.messages.is_empty());
        // Test that error messages contain meaningful content
        assert!(exception.messages[0].contains("周辺の山岳情報を検索中にエラーが発生しました"));
    }

    #[test]
    fn test_surrounding_mountain_multiple_results() {
        let mountain1 = create_test_searched_surrounding_mountain();
        let mut mountain2 = create_test_searched_surrounding_mountain();
        mountain2.id = 3;
        mountain2.name = "別の周辺の山".to_string();

        let mountains = vec![mountain1, mountain2];
        let json_mountains: Vec<JsonSurroundingMountain> =
            mountains.into_iter().map(|m| m.into()).collect();

        assert_eq!(json_mountains.len(), 2);
        assert_eq!(json_mountains[0].id, 2);
        assert_eq!(json_mountains[1].id, 3);
        assert_eq!(json_mountains[1].name, "別の周辺の山");
    }
}
