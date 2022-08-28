use crate::model::invalid_param_error;
use mountix_kernel::model::surrounding_mountain::{
    SurroundingMountain, SurroundingMountainLocation, SurroundingMountainSearchDistance,
};
use std::env;
use std::ffi::OsString;

pub struct SearchedSurroundingMountain {
    pub id: i32,
    pub name: String,
    pub name_kana: String,
    pub area: String,
    pub prefectures: Vec<String>,
    pub elevation: u32,
    pub location: SearchedSurroundingMountainLocation,
    pub tags: Vec<String>,
}

impl From<SurroundingMountain> for SearchedSurroundingMountain {
    fn from(mountain: SurroundingMountain) -> Self {
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

pub struct SearchedSurroundingMountainLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub gsi_url: String,
}

impl From<SurroundingMountainLocation> for SearchedSurroundingMountainLocation {
    fn from(location: SurroundingMountainLocation) -> Self {
        Self {
            latitude: location.latitude,
            longitude: location.longitude,
            gsi_url: location.gsi_url,
        }
    }
}

pub struct SearchedSurroundingMountainResult {
    pub mountains: Vec<SearchedSurroundingMountain>,
    pub distance: u32,
}

pub struct SurroundingMountainSearchQuery {
    pub distance: Option<String>,
}

impl TryFrom<SurroundingMountainSearchQuery> for SurroundingMountainSearchDistance {
    type Error = Vec<String>;

    fn try_from(query: SurroundingMountainSearchQuery) -> Result<Self, Self::Error> {
        let env_max_distance = env::var_os("MAX_DISTANCE").unwrap_or(OsString::from("100000"));
        let max_distance = env_max_distance
            .into_string()
            .unwrap_or("100000".to_string())
            .parse::<u32>()
            .unwrap_or(100_000);

        match query.distance {
            Some(query_distance) => match query_distance.parse::<u32>() {
                Ok(distance) => {
                    if distance <= max_distance {
                        Ok(SurroundingMountainSearchDistance::new(distance))
                    } else {
                        Err(vec![invalid_param_error("distance")])
                    }
                }
                Err(_) => Err(vec![invalid_param_error("distance")]),
            },
            None => Ok(Default::default()),
        }
    }
}
