use crate::model::mountain::Mountain;
use crate::model::{ErrorCode, Id};
use std::env;
use std::ffi::OsString;

const ERR_MESSAGE_SURROUNDING_FIND_EXCEPTION: &str =
    "周辺の山岳情報を検索中にエラーが発生しました。";

#[derive(Debug)]
pub struct SurroundingMountain {
    pub id: Id<SurroundingMountain>,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: SurroundingMountainLocation,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct SurroundingMountainLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

pub struct SurroundingMountainData {
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: SurroundingMountainLocation,
    pub tags: Vec<String>,
}

impl SurroundingMountain {
    pub fn new(id: Id<SurroundingMountain>, data: SurroundingMountainData) -> Self {
        Self {
            id,
            name: data.name,
            name_kana: data.name_kana,
            area: data.area,
            prefectures: data.prefectures,
            elevation: data.elevation,
            location: data.location,
            tags: data.tags,
        }
    }
}

impl SurroundingMountainLocation {
    pub fn new(latitude: f64, longitude: f64, gsi_url: String) -> Self {
        Self {
            latitude,
            longitude,
            gsi_url,
        }
    }
}

pub struct SurroundingMountainSearchDistance(pub u32);

impl SurroundingMountainSearchDistance {
    pub fn new(distance: u32) -> Self {
        Self(distance)
    }
}

impl Default for SurroundingMountainSearchDistance {
    fn default() -> Self {
        let env_default_distance =
            env::var_os("DEFAULT_DISTANCE").unwrap_or(OsString::from("5000"));
        let distance = env_default_distance
            .into_string()
            .unwrap_or("5000".to_string())
            .parse::<u32>()
            .unwrap_or(5_000);

        Self(distance)
    }
}

pub struct SurroundingMountainSearchCondition {
    pub mountain: Mountain,
    pub distance: SurroundingMountainSearchDistance,
}

impl SurroundingMountainSearchCondition {
    pub fn new(mountain: Mountain, distance: SurroundingMountainSearchDistance) -> Self {
        Self { mountain, distance }
    }
}

#[derive(Debug)]
pub struct SurroundingMountainFindException {
    pub error_code: ErrorCode,
    pub messages: Vec<String>,
}

impl SurroundingMountainFindException {
    pub fn new(error_code: ErrorCode, messages: Vec<String>) -> Self {
        Self {
            error_code,
            messages,
        }
    }

    /// Returns an error including exception error messages
    ///
    /// 周辺の山岳を検索時の例外エラーメッセージを含むエラーを生成します
    ///
    /// # Arguments
    ///
    /// - `error_code`: Error code
    pub fn new_with_error_code(error_code: ErrorCode) -> Self {
        let messages = vec![ERR_MESSAGE_SURROUNDING_FIND_EXCEPTION.to_string()];
        Self {
            error_code,
            messages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::mountain::{Mountain, MountainLocation};

    fn create_test_mountain() -> Mountain {
        let id = Id::new(1);
        let location =
            MountainLocation::new(35.360556, 138.727778, "https://maps.gsi.go.jp".to_string());
        let data = crate::model::mountain::MountainData {
            name: "富士山".to_string(),
            name_kana: "ふじさん".to_string(),
            area: "関東地方".to_string(),
            prefectures: vec!["静岡県".to_string(), "山梨県".to_string()],
            elevation: 3776,
            location,
            tags: vec!["百名山".to_string()],
        };
        Mountain::new(id, data)
    }

    #[test]
    fn test_surrounding_mountain_search_distance_default() {
        let distance = SurroundingMountainSearchDistance::default();
        assert_eq!(distance.0, 5000);
    }

    #[test]
    fn test_surrounding_mountain_search_distance_with_env_var() {
        std::env::set_var("DEFAULT_DISTANCE", "8000");
        let distance = SurroundingMountainSearchDistance::default();
        assert_eq!(distance.0, 8000);
        std::env::remove_var("DEFAULT_DISTANCE");
    }

    #[test]
    fn test_surrounding_mountain_search_distance_with_invalid_env_var() {
        std::env::set_var("DEFAULT_DISTANCE", "invalid");
        let distance = SurroundingMountainSearchDistance::default();
        assert_eq!(distance.0, 5000);
        std::env::remove_var("DEFAULT_DISTANCE");
    }

    #[test]
    fn test_surrounding_mountain_search_condition_new() {
        let mountain = create_test_mountain();
        let distance = SurroundingMountainSearchDistance::new(15000);
        let condition = SurroundingMountainSearchCondition::new(mountain, distance);

        assert_eq!(condition.mountain.id.value, 1);
        assert_eq!(condition.distance.0, 15000);
    }

    #[test]
    fn test_surrounding_mountain_find_exception_new_with_multiple_messages() {
        let messages = vec!["First error".to_string(), "Second error".to_string()];
        let exception =
            SurroundingMountainFindException::new(ErrorCode::InvalidQueryParam, messages.clone());

        assert_eq!(exception.error_code, ErrorCode::InvalidQueryParam);
        assert_eq!(exception.messages.len(), 2);
        assert_eq!(exception.messages[0], "First error");
        assert_eq!(exception.messages[1], "Second error");
    }
}
