#[derive(Debug)]
pub struct Mountain {
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: Location,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

impl Mountain {
    pub fn new(
        id: i32,
        name: String,
        name_kana: String,
        area: String,
        prefectures: Vec<String>,
        elevation: u32,
        location: Location,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            name_kana,
            area,
            prefectures,
            elevation,
            location,
            tags,
        }
    }
}

impl Location {
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
    pub tag: Option<MountainsTag>,
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
pub struct MountainsTag {
    pub id: u64,
    pub name: String,
}

impl MountainsTag {
    const TAGS: [(u64, &'static str); 2] = [(1, "百名山"), (2, "二百名山")];

    fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

impl TryFrom<String> for MountainsTag {
    type Error = anyhow::Error;

    fn try_from(tag_param: String) -> Result<Self, Self::Error> {
        match tag_param.parse::<u64>() {
            Ok(tag_id) => {
                for tag in MountainsTag::TAGS {
                    if tag.0 == tag_id {
                        return Ok(MountainsTag::new(tag.0, tag.1.to_string()));
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
    pub fn to_type(&self) -> i64 {
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
    type Error = anyhow::Error;

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
            _ => Err(Self::Error::msg("Invalid sort value.")),
        }
    }
}
