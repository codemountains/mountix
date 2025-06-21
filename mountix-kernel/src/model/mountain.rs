use crate::model::{ErrorCode, Id};
use regex::Regex;

const ERR_MESSAGE_FIND_EXCEPTION: &str = "山岳情報を検索中にエラーが発生しました。";

#[derive(Debug)]
pub struct Mountain {
    pub id: Id<Mountain>,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: MountainLocation,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct MountainLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

pub struct MountainData {
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: MountainLocation,
    pub tags: Vec<String>,
}

impl Mountain {
    pub fn new(id: Id<Mountain>, data: MountainData) -> Self {
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

impl MountainLocation {
    pub fn new(latitude: f64, longitude: f64, gsi_url: String) -> Self {
        Self {
            latitude,
            longitude,
            gsi_url,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MountainSearchCondition {
    pub name: Option<String>,
    pub prefecture: Option<MountainPrefecture>,
    pub tag: Option<MountainTag>,
    pub skip: u64,
    pub limit: Option<i64>,
    pub sort: MountainSortCondition,
}

#[derive(Debug, Clone)]
pub struct MountainPrefecture {
    pub id: u64,
    pub name: String,
}

impl MountainPrefecture {
    const PREFECTURES: [(u64, &'static str); 47] = [
        (1, "北海道"),
        (2, "青森県"),
        (3, "岩手県"),
        (4, "宮城県"),
        (5, "秋田県"),
        (6, "山形県"),
        (7, "福島県"),
        (8, "茨城県"),
        (9, "栃木県"),
        (10, "群馬県"),
        (11, "埼玉県"),
        (12, "千葉県"),
        (13, "東京都"),
        (14, "神奈川県"),
        (15, "新潟県"),
        (16, "富山県"),
        (17, "石川県"),
        (18, "福井県"),
        (19, "山梨県"),
        (20, "長野県"),
        (21, "岐阜県"),
        (22, "静岡県"),
        (23, "愛知県"),
        (24, "三重県"),
        (25, "滋賀県"),
        (26, "京都府"),
        (27, "大阪府"),
        (28, "兵庫県"),
        (29, "奈良県"),
        (30, "和歌山県"),
        (31, "鳥取県"),
        (32, "島根県"),
        (33, "岡山県"),
        (34, "広島県"),
        (35, "山口県"),
        (36, "徳島県"),
        (37, "香川県"),
        (38, "愛媛県"),
        (39, "高知県"),
        (40, "福岡県"),
        (41, "佐賀県"),
        (42, "長崎県"),
        (43, "熊本県"),
        (44, "大分県"),
        (45, "宮崎県"),
        (46, "鹿児島県"),
        (47, "沖縄県"),
    ];

    fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

impl TryFrom<String> for MountainPrefecture {
    type Error = anyhow::Error;

    fn try_from(prefecture_param: String) -> Result<Self, Self::Error> {
        match prefecture_param.parse::<u64>() {
            Ok(prefecture_id) => {
                for pref in MountainPrefecture::PREFECTURES {
                    if pref.0 == prefecture_id {
                        return Ok(MountainPrefecture::new(pref.0, pref.1.to_string()));
                    }
                }
                Err(Self::Error::msg("Invalid prefecture value."))
            }
            Err(_) => Err(Self::Error::msg("Invalid prefecture value.")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MountainTag {
    pub id: u64,
    pub name: String,
}

impl MountainTag {
    const TAGS: [(u64, &'static str); 2] = [(1, "百名山"), (2, "二百名山")];

    fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

impl TryFrom<String> for MountainTag {
    type Error = anyhow::Error;

    fn try_from(tag_param: String) -> Result<Self, Self::Error> {
        match tag_param.parse::<u64>() {
            Ok(tag_id) => {
                for tag in MountainTag::TAGS {
                    if tag.0 == tag_id {
                        return Ok(MountainTag::new(tag.0, tag.1.to_string()));
                    }
                }
                Err(Self::Error::msg("Invalid tag value."))
            }
            Err(_) => Err(Self::Error::msg("Invalid tag value.")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MountainSortKey {
    Id,
    Elevation,
    Name,
}

impl MountainSortKey {
    pub fn to_key(&self) -> String {
        match self {
            MountainSortKey::Id => "_id".to_string(),
            MountainSortKey::Elevation => "elevation".to_string(),
            MountainSortKey::Name => "name_kana".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MountainOrderType {
    Asc,
    Desc,
}

impl MountainOrderType {
    pub fn to_value(&self) -> i64 {
        match self {
            MountainOrderType::Asc => 1,
            MountainOrderType::Desc => -1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MountainSortCondition {
    pub key: MountainSortKey,
    pub order: MountainOrderType,
}

impl Default for MountainSortCondition {
    fn default() -> Self {
        Self {
            key: MountainSortKey::Id,
            order: MountainOrderType::Asc,
        }
    }
}

impl TryFrom<String> for MountainSortCondition {
    type Error = ();

    fn try_from(sort_param: String) -> Result<Self, Self::Error> {
        match sort_param.as_str() {
            "id.asc" => Ok(MountainSortCondition {
                key: MountainSortKey::Id,
                order: MountainOrderType::Asc,
            }),
            "id.desc" => Ok(MountainSortCondition {
                key: MountainSortKey::Id,
                order: MountainOrderType::Desc,
            }),
            "elevation.asc" => Ok(MountainSortCondition {
                key: MountainSortKey::Elevation,
                order: MountainOrderType::Asc,
            }),
            "elevation.desc" => Ok(MountainSortCondition {
                key: MountainSortKey::Elevation,
                order: MountainOrderType::Desc,
            }),
            "name.asc" => Ok(MountainSortCondition {
                key: MountainSortKey::Name,
                order: MountainOrderType::Asc,
            }),
            "name.desc" => Ok(MountainSortCondition {
                key: MountainSortKey::Name,
                order: MountainOrderType::Desc,
            }),
            _ => Err(()),
        }
    }
}

pub struct MountainBoxSearchCondition {
    pub box_coordinates: MountainBoxCoordinates,
    pub name: Option<String>,
    pub tag: Option<MountainTag>,
    pub sort: MountainSortCondition,
}

#[derive(Debug)]
pub struct MountainBoxCoordinates {
    pub bottom_left: (f64, f64),
    pub upper_right: (f64, f64),
}

impl TryFrom<String> for MountainBoxCoordinates {
    type Error = anyhow::Error;

    fn try_from(box_param: String) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"\(([+-]?\d+(?:\.\d+)?),([+-]?\d+(?:\.\d+)?)\),\(([+-]?\d+(?:\.\d+)?),([+-]?\d+(?:\.\d+)?)\)",
        )?;

        let caps = re
            .captures(box_param.as_str())
            .ok_or(Self::Error::msg("Invalid box parameter."))?;

        let bottom_left_lng = caps
            .get(1)
            .ok_or(Self::Error::msg("Invalid bottom left longitude."))?
            .as_str()
            .parse::<f64>()?;
        if !(-180.0..=180.0).contains(&bottom_left_lng) {
            return Err(Self::Error::msg("Invalid bottom left longitude."));
        }

        let bottom_left_lat = caps
            .get(2)
            .ok_or(Self::Error::msg("Invalid bottom left latitude."))?
            .as_str()
            .parse::<f64>()?;
        if !(-90.0..=90.0).contains(&bottom_left_lat) {
            return Err(Self::Error::msg("Invalid bottom left latitude."));
        }

        let upper_right_lng = caps
            .get(3)
            .ok_or(Self::Error::msg("Invalid upper right longitude."))?
            .as_str()
            .parse::<f64>()?;
        if !(-180.0..=180.0).contains(&upper_right_lng) {
            return Err(Self::Error::msg("Invalid upper right longitude."));
        }

        let upper_right_lat = caps
            .get(4)
            .ok_or(Self::Error::msg("Invalid upper right latitude."))?
            .as_str()
            .parse::<f64>()?;
        if !(-90.0..=90.0).contains(&upper_right_lat) {
            return Err(Self::Error::msg("Invalid upper right latitude."));
        }

        Ok(MountainBoxCoordinates {
            bottom_left: (bottom_left_lng, bottom_left_lat),
            upper_right: (upper_right_lng, upper_right_lat),
        })
    }
}

#[derive(Debug)]
pub struct MountainGetException {
    pub error_code: ErrorCode,
}

impl MountainGetException {
    pub fn new(error_code: ErrorCode) -> Self {
        Self { error_code }
    }
}

#[derive(Debug)]
pub struct MountainFindException {
    pub error_code: ErrorCode,
    pub messages: Vec<String>,
}

impl MountainFindException {
    pub fn new(error_code: ErrorCode, messages: Vec<String>) -> Self {
        Self {
            error_code,
            messages,
        }
    }

    /// Returns an error including exception error messages
    ///
    /// 検索時の例外エラーメッセージを含むエラーを生成します
    ///
    /// # Arguments
    ///
    /// - `error_code`: Error code
    pub fn new_with_error_code(error_code: ErrorCode) -> Self {
        let messages = vec![ERR_MESSAGE_FIND_EXCEPTION.to_string()];
        Self {
            error_code,
            messages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mountain_prefecture_try_from_valid_id() {
        let result = MountainPrefecture::try_from("1".to_string());
        assert!(result.is_ok());
        let prefecture = result.unwrap();
        assert_eq!(prefecture.id, 1);
        assert_eq!(prefecture.name, "北海道");
    }

    #[test]
    fn test_mountain_prefecture_try_from_valid_tokyo() {
        let result = MountainPrefecture::try_from("13".to_string());
        assert!(result.is_ok());
        let prefecture = result.unwrap();
        assert_eq!(prefecture.id, 13);
        assert_eq!(prefecture.name, "東京都");
    }

    #[test]
    fn test_mountain_prefecture_try_from_invalid_id() {
        let result = MountainPrefecture::try_from("48".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid prefecture value.");
    }

    #[test]
    fn test_mountain_prefecture_try_from_invalid_string() {
        let result = MountainPrefecture::try_from("invalid".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid prefecture value.");
    }

    #[test]
    fn test_mountain_prefecture_try_from_zero() {
        let result = MountainPrefecture::try_from("0".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid prefecture value.");
    }

    #[test]
    fn test_mountain_tag_try_from_valid_hyakumeizan() {
        let result = MountainTag::try_from("1".to_string());
        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.id, 1);
        assert_eq!(tag.name, "百名山");
    }

    #[test]
    fn test_mountain_tag_try_from_valid_nihyakumeizan() {
        let result = MountainTag::try_from("2".to_string());
        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.id, 2);
        assert_eq!(tag.name, "二百名山");
    }

    #[test]
    fn test_mountain_tag_try_from_invalid_id() {
        let result = MountainTag::try_from("3".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid tag value.");
    }

    #[test]
    fn test_mountain_tag_try_from_invalid_string() {
        let result = MountainTag::try_from("invalid".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid tag value.");
    }

    #[test]
    fn test_mountain_tag_try_from_zero() {
        let result = MountainTag::try_from("0".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid tag value.");
    }

    #[test]
    fn test_mountain_sort_condition_try_from_id_asc() {
        let result = MountainSortCondition::try_from("id.asc".to_string());
        assert!(result.is_ok());
        let sort = result.unwrap();
        assert_eq!(sort.key.to_key(), "_id");
        assert_eq!(sort.order.to_value(), 1);
    }

    #[test]
    fn test_mountain_sort_condition_try_from_id_desc() {
        let result = MountainSortCondition::try_from("id.desc".to_string());
        assert!(result.is_ok());
        let sort = result.unwrap();
        assert_eq!(sort.key.to_key(), "_id");
        assert_eq!(sort.order.to_value(), -1);
    }

    #[test]
    fn test_mountain_sort_condition_try_from_elevation_asc() {
        let result = MountainSortCondition::try_from("elevation.asc".to_string());
        assert!(result.is_ok());
        let sort = result.unwrap();
        assert_eq!(sort.key.to_key(), "elevation");
        assert_eq!(sort.order.to_value(), 1);
    }

    #[test]
    fn test_mountain_sort_condition_try_from_name_desc() {
        let result = MountainSortCondition::try_from("name.desc".to_string());
        assert!(result.is_ok());
        let sort = result.unwrap();
        assert_eq!(sort.key.to_key(), "name_kana");
        assert_eq!(sort.order.to_value(), -1);
    }

    #[test]
    fn test_mountain_sort_condition_try_from_invalid() {
        let result = MountainSortCondition::try_from("invalid".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_mountain_sort_condition_default() {
        let sort = MountainSortCondition::default();
        assert_eq!(sort.key.to_key(), "_id");
        assert_eq!(sort.order.to_value(), 1);
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_valid() {
        let box_param = "(139.0,35.0),(140.0,36.0)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_ok());
        let coords = result.unwrap();
        assert_eq!(coords.bottom_left, (139.0, 35.0));
        assert_eq!(coords.upper_right, (140.0, 36.0));
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_negative_coords() {
        let box_param = "(-139.5,-35.5),(-138.5,-34.5)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_ok());
        let coords = result.unwrap();
        assert_eq!(coords.bottom_left, (-139.5, -35.5));
        assert_eq!(coords.upper_right, (-138.5, -34.5));
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_boundary_longitude() {
        let box_param = "(-180.0,35.0),(180.0,36.0)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_ok());
        let coords = result.unwrap();
        assert_eq!(coords.bottom_left, (-180.0, 35.0));
        assert_eq!(coords.upper_right, (180.0, 36.0));
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_boundary_latitude() {
        let box_param = "(139.0,-90.0),(140.0,90.0)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_ok());
        let coords = result.unwrap();
        assert_eq!(coords.bottom_left, (139.0, -90.0));
        assert_eq!(coords.upper_right, (140.0, 90.0));
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_invalid_longitude() {
        let box_param = "(181.0,35.0),(140.0,36.0)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid bottom left longitude."
        );
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_invalid_latitude() {
        let box_param = "(139.0,91.0),(140.0,36.0)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid bottom left latitude."
        );
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_invalid_format() {
        let box_param = "invalid_format".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid box parameter.");
    }

    #[test]
    fn test_mountain_box_coordinates_try_from_missing_coordinates() {
        let box_param = "(139.0,35.0)".to_string();
        let result = MountainBoxCoordinates::try_from(box_param);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid box parameter.");
    }
}
