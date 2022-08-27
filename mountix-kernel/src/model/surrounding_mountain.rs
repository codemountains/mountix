use crate::model::mountain::Mountain;
use crate::model::{ErrorCode, Id};
use std::env;
use std::ffi::OsString;

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

impl SurroundingMountain {
    pub fn new(
        id: Id<SurroundingMountain>,
        name: String,
        name_kana: String,
        area: String,
        prefectures: Vec<String>,
        elevation: u32,
        location: SurroundingMountainLocation,
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
}
