use mountix_kernel::model::mountain::{
    Location, Mountain, MountainPrefecture, MountainSearchCondition, MountainSortCondition,
    MountainsTag,
};

pub struct SearchedMountain {
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: SearchedLocation,
    pub tags: Vec<String>,
}

impl From<Mountain> for SearchedMountain {
    fn from(mountain: Mountain) -> Self {
        Self {
            id: mountain.id.value,
            name: mountain.name,
            name_kana: mountain.name_kana,
            area: mountain.area,
            prefectures: mountain.prefectures,
            elevation: mountain.elevation,
            location: mountain.location.into(),
            tags: mountain.tags,
        }
    }
}

pub struct SearchedLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

impl From<Location> for SearchedLocation {
    fn from(location: Location) -> Self {
        Self {
            latitude: location.latitude,
            longitude: location.longitude,
            gsi_url: location.gsi_url,
        }
    }
}

pub struct FoundMountains {
    pub mountains: Vec<SearchedMountain>,
    pub total: u64,
    pub offset: u64,
    pub limit: Option<u64>,
}

pub struct MountainSearchQuery {
    pub name: Option<String>,
    pub prefecture: Option<String>,
    pub tag: Option<String>,
    pub offset: Option<String>,
    pub limit: Option<String>,
    pub sort: Option<String>,
}

impl TryFrom<MountainSearchQuery> for MountainSearchCondition {
    type Error = Vec<String>;

    fn try_from(ms: MountainSearchQuery) -> Result<Self, Self::Error> {
        let mut errors: Vec<String> = Vec::new();

        let name = ms.name;

        let mut prefecture: Option<MountainPrefecture> = None;
        if let Some(prefecture_param) = ms.prefecture {
            match MountainPrefecture::try_from(prefecture_param) {
                Ok(p) => prefecture = Some(p),
                Err(_) => errors.push("都道府県IDが不正です。".to_string()),
            }
        };

        let mut tag: Option<MountainsTag> = None;
        if let Some(tag_param) = ms.tag {
            match MountainsTag::try_from(tag_param) {
                Ok(t) => tag = Some(t),
                Err(_) => errors.push("タグIDが不正です。".to_string()),
            }
        }

        let mut sort: MountainSortCondition = Default::default();
        if let Some(sort_param) = ms.sort {
            match MountainSortCondition::try_from(sort_param) {
                Ok(s) => sort = s,
                Err(_) => errors.push("ソートの値が不正です。".to_string()),
            }
        }

        let mut skip = 0u64;
        if let Some(offset_param) = ms.offset {
            match offset_param.parse::<u64>() {
                Ok(skip_value) => skip = skip_value,
                Err(_) => {
                    errors.push("オフセットの値が不正です。".to_string());
                }
            }
        }

        let mut limit: Option<i64> = None;
        if let Some(limit_param) = ms.limit {
            match limit_param.parse::<i64>() {
                Ok(limit_value) => {
                    if limit_value < 0 {
                        errors.push("リミットの値が不正です。".to_string());
                    }
                    limit = Some(limit_value)
                }
                Err(_) => {
                    errors.push("リミットの値が不正です。".to_string());
                }
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(MountainSearchCondition {
            name,
            prefecture,
            tag,
            skip,
            limit,
            sort,
        })
    }
}
