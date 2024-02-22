#![allow(dead_code)]

use std::io::prelude::*;

use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

mod blueprint;
mod book;
mod planner;

pub use blueprint::*;
pub use book::*;
pub use planner::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonData<T> {
    #[serde(flatten)]
    data: T,

    pub item: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,

    pub version: u64, // see https://wiki.factorio.com/Version_string_format
}

impl<T> CommonData<T> {
    #[must_use]
    pub fn version_string(&self) -> String {
        let major = self.version >> (64 - 2 * 8);
        let minor = self.version >> (64 - 4 * 8) & 0xFF;
        let patch = self.version >> (64 - 6 * 8) & 0xFF;
        //let dev = self.version & 0xFF;

        format!("{major}.{minor}.{patch}") //-{dev}")
    }
}

impl<T> std::ops::Deref for CommonData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for CommonData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Indexed<T> {
    pub index: u16,

    #[serde(flatten)]
    data: T,
}

pub type IndexedVec<T> = Vec<Indexed<T>>;

impl<T> std::ops::Deref for Indexed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Indexed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct NameString {
    name: String,
}

impl std::ops::Deref for NameString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl std::fmt::Display for NameString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Data {
    Blueprint(Blueprint),
    BlueprintBook(Book),
    UpgradePlanner(UpgradePlanner),
    DeconstructionPlanner(DeconPlanner),
}

impl Data {
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Blueprint(data) => &data.label,
            Self::BlueprintBook(data) => &data.label,
            Self::UpgradePlanner(data) => &data.label,
            Self::DeconstructionPlanner(data) => &data.label,
        }
    }

    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::Blueprint(data) => &data.description,
            Self::BlueprintBook(data) => &data.description,
            Self::UpgradePlanner(data) => &data.description,
            Self::DeconstructionPlanner(data) => &data.description,
        }
    }

    #[must_use]
    pub fn item(&self) -> &str {
        match self {
            Self::Blueprint(data) => &data.item,
            Self::BlueprintBook(data) => &data.item,
            Self::UpgradePlanner(data) => &data.item,
            Self::DeconstructionPlanner(data) => &data.item,
        }
    }

    #[must_use]
    pub fn icons(&self) -> &[Indexed<Icon>] {
        match self {
            Self::Blueprint(data) => &data.icons,
            Self::BlueprintBook(data) => &data.icons,
            Self::UpgradePlanner(data) => &data.icons,
            Self::DeconstructionPlanner(data) => &data.icons,
        }
    }

    #[must_use]
    pub const fn is_book(&self) -> bool {
        matches!(self, Self::BlueprintBook { .. })
    }

    #[must_use]
    pub const fn is_blueprint(&self) -> bool {
        matches!(self, Self::Blueprint { .. })
    }

    #[must_use]
    pub const fn as_book(&self) -> Option<&Book> {
        match self {
            Self::BlueprintBook(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_book_mut(&mut self) -> Option<&mut Book> {
        match self {
            Self::BlueprintBook(data) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_blueprint(&self) -> Option<&Blueprint> {
        match self {
            Self::Blueprint(data) => Some(data),
            Self::BlueprintBook(book) if !book.blueprints.is_empty() => book
                .blueprints
                .iter()
                .find(|entry| entry.index == book.active_index)
                .and_then(|entry| entry.data.as_blueprint()),
            _ => None,
        }
    }

    pub fn as_blueprint_mut(&mut self) -> Option<&mut Blueprint> {
        match self {
            Self::Blueprint(data) => Some(data),
            Self::BlueprintBook(book) if !book.blueprints.is_empty() => {
                let index = book.active_index;
                book.blueprints
                    .iter_mut()
                    .find(|entry| entry.index == index)
                    .and_then(|entry| entry.data.as_blueprint_mut())
            }
            _ => None,
        }
    }

    fn normalize_positions(&mut self) {
        match self {
            Self::BlueprintBook(data) => {
                for entry in &mut data.blueprints {
                    entry.data.normalize_positions();
                }
            }
            Self::Blueprint(data) => {
                let mut min_x = f32::MAX;
                let mut min_y = f32::MAX;
                let mut max_x = f32::MIN;
                let mut max_y = f32::MIN;

                for entity in &*data.entities {
                    min_x = min_x.min(entity.position.x);
                    min_y = min_y.min(entity.position.y);
                    max_x = max_x.max(entity.position.x);
                    max_y = max_y.max(entity.position.y);
                }

                for tile in &*data.tiles {
                    min_x = min_x.min(tile.position.x);
                    min_y = min_y.min(tile.position.y);
                    max_x = max_x.max(tile.position.x);
                    max_y = max_y.max(tile.position.y);
                }

                let width = ((max_x - min_x) / 2.0).round();
                let height = ((max_y - min_y) / 2.0).round();
                let offset_x = (min_x + width).round();
                let offset_y = (min_y + height).round();

                // only offset an even amount
                let offset_x = if offset_x % 2.0 == 0.0 {
                    offset_x
                } else {
                    offset_x - 1.0
                };

                let offset_y = if offset_y % 2.0 == 0.0 {
                    offset_y
                } else {
                    offset_y - 1.0
                };

                println!("normalize offset: {offset_x}, {offset_y}");

                for entity in &mut data.entities {
                    entity.position.x -= offset_x;
                    entity.position.y -= offset_y;
                }

                for tile in &mut data.tiles {
                    tile.position.x -= offset_x;
                    tile.position.y -= offset_y;
                }
            }
            _ => {}
        }
    }

    fn ensure_ordering(&mut self) {
        match self {
            Self::BlueprintBook(data) => {
                for entry in &mut data.blueprints {
                    entry.data.ensure_ordering();
                }
            }
            Self::Blueprint(data) => {
                data.entities
                    .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                data.tiles
                    .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            }
            // TODO: sort ordering for upgrade planner and deconstruction planner
            _ => {}
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlueprintDecodeError {
    #[error("blueprint string must be at least 2 characters long")]
    MinSize,

    #[error("unsupported blueprint version: {0}")]
    UnsupportedVersion(char),

    #[error("blueprint string parsing failed")]
    Parsing,

    #[error("blueprint string decoding failed: {0}")]
    Decoding(#[from] base64::DecodeError),

    #[error("blueprint string decompression failed: {0}")]
    Decompress(#[from] std::io::Error),

    #[error("blueprint string deserialization failed: {0}")]
    Deserializing(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum BlueprintEncodeError {
    #[error("blueprint string compression failed: {0}")]
    Decompress(#[from] std::io::Error),

    #[error("blueprint string serialization failed: {0}")]
    Serializing(#[from] serde_json::Error),
}

pub fn bp_string_to_json(bp_string: &str) -> Result<String, BlueprintDecodeError> {
    if bp_string.len() < 2 {
        return Err(BlueprintDecodeError::MinSize);
    }

    let mut chars = bp_string.chars();

    match chars.next() {
        Some(first) => {
            if first != '0' {
                return Err(BlueprintDecodeError::UnsupportedVersion(first));
            }
        }
        None => return Err(BlueprintDecodeError::Parsing),
    }

    let compressed = general_purpose::STANDARD.decode(chars.as_str())?;

    let mut deflate = ZlibDecoder::new(compressed.as_slice());
    let mut uncompressed = String::new();

    deflate.read_to_string(&mut uncompressed)?;
    // .map_err(|err| Err(BlueprintError::Decompressing(err.to_string())))?;

    Ok(uncompressed)
}

pub fn json_to_bp_string(json: &str) -> Result<String, BlueprintEncodeError> {
    let mut deflate = ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    deflate.write_all(json.as_bytes())?;
    let compressed = deflate.finish()?;

    let mut encoded = general_purpose::STANDARD.encode(compressed);

    encoded.insert(0, '0');

    Ok(encoded)
}

impl TryFrom<&str> for Data {
    type Error = BlueprintDecodeError;

    fn try_from(bp_string: &str) -> Result<Self, Self::Error> {
        let json = bp_string_to_json(bp_string)?;
        let mut data: Self = serde_json::from_str(&json)?;

        data.normalize_positions();
        data.ensure_ordering();

        Ok(data)
    }
}

impl TryFrom<String> for Data {
    type Error = BlueprintDecodeError;

    fn try_from(bp_string: String) -> Result<Self, Self::Error> {
        Self::try_from(bp_string.as_str())
    }
}

impl TryFrom<Data> for String {
    type Error = BlueprintEncodeError;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&data)?;

        json_to_bp_string(&json)
    }
}
