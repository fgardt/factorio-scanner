#![allow(dead_code)]

use std::{collections::HashSet, io::prelude::*};

use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::{debug, instrument};

mod blueprint;
mod book;
mod planner;

pub use blueprint::*;
pub use book::*;
pub use planner::*;
use types::{
    AsteroidChunkID, EntityID, FluidID, ItemID, QualityID, RecipeID, SpaceLocationID, TileID,
    VirtualSignalID,
};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct UsedIDs {
    pub recipe: HashSet<RecipeID>,
    pub entity: HashSet<EntityID>,
    pub tile: HashSet<TileID>,
    pub fluid: HashSet<FluidID>,
    pub item: HashSet<ItemID>,
    pub virtual_signal: HashSet<VirtualSignalID>,
    pub quality: HashSet<QualityID>,
    pub space_location: HashSet<SpaceLocationID>,
    pub asteroid_chunk: HashSet<AsteroidChunkID>,
}

impl UsedIDs {
    pub fn merge(&mut self, other: Self) {
        self.recipe.extend(other.recipe);
        self.entity.extend(other.entity);
        self.tile.extend(other.tile);
        self.fluid.extend(other.fluid);
        self.item.extend(other.item);
        self.virtual_signal.extend(other.virtual_signal);
        self.quality.extend(other.quality);
        self.space_location.extend(other.space_location);
        self.asteroid_chunk.extend(other.asteroid_chunk);
    }
}

pub trait GetIDs {
    fn get_ids(&self) -> UsedIDs;
}

impl<T: GetIDs> GetIDs for Vec<T> {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        for entry in self {
            ids.merge(entry.get_ids());
        }

        ids
    }
}

impl<T: GetIDs> GetIDs for Box<T> {
    fn get_ids(&self) -> crate::UsedIDs {
        self.as_ref().get_ids()
    }
}

/// see <https://wiki.factorio.com/Version_string_format>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Version(u64);

impl Version {
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16, dev: u16) -> Self {
        Self(
            (major as u64) << (6 * 8)
                | (minor as u64) << (4 * 8)
                | (patch as u64) << (2 * 8)
                | (dev as u64),
        )
    }

    #[must_use]
    pub const fn major(&self) -> u16 {
        (self.0 >> (6 * 8)) as u16
    }

    #[must_use]
    pub const fn minor(&self) -> u16 {
        (self.0 >> (4 * 8) & 0xFF) as u16
    }

    #[must_use]
    pub const fn patch(&self) -> u16 {
        (self.0 >> (2 * 8) & 0xFF) as u16
    }

    #[must_use]
    pub const fn dev(&self) -> u16 {
        (self.0 & 0xFF) as u16
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Version {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        let s_major = self.major();
        let o_major = other.major();

        if s_major != o_major {
            return s_major.cmp(&o_major);
        }

        let s_minor = self.minor();
        let o_minor = other.minor();

        if s_minor != o_minor {
            return s_minor.cmp(&o_minor);
        }

        let s_patch = self.patch();
        let o_patch = other.patch();
        s_patch.cmp(&o_patch)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonData<T> {
    #[serde(flatten)]
    data: T,

    pub item: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,

    pub version: Version,
}

impl TryFrom<Blueprint> for String {
    type Error = BlueprintEncodeError;

    fn try_from(data: Blueprint) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&Data::Blueprint(data))?;
        json_to_bp_string(&json)
    }
}

impl TryFrom<Book> for String {
    type Error = BlueprintEncodeError;

    fn try_from(data: Book) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&Data::BlueprintBook(data))?;
        json_to_bp_string(&json)
    }
}

impl TryFrom<UpgradePlanner> for String {
    type Error = BlueprintEncodeError;

    fn try_from(data: UpgradePlanner) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&Data::UpgradePlanner(data))?;
        json_to_bp_string(&json)
    }
}

impl TryFrom<DeconPlanner> for String {
    type Error = BlueprintEncodeError;

    fn try_from(data: DeconPlanner) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&Data::DeconstructionPlanner(data))?;
        json_to_bp_string(&json)
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

impl<T: GetIDs> GetIDs for Indexed<T> {
    fn get_ids(&self) -> crate::UsedIDs {
        self.data.get_ids()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct NameString<T> {
    name: T,
}

impl<T> std::ops::Deref for NameString<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl<T> std::fmt::Display for NameString<T>
where
    T: std::fmt::Display,
{
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
    pub const fn version(&self) -> Version {
        match self {
            Self::Blueprint(data) => data.version,
            Self::BlueprintBook(data) => data.version,
            Self::UpgradePlanner(data) => data.version,
            Self::DeconstructionPlanner(data) => data.version,
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

                debug!("normalize offset: {offset_x}, {offset_y}");

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

impl GetIDs for Data {
    fn get_ids(&self) -> UsedIDs {
        match self {
            Self::Blueprint(data) => data.get_ids(),
            Self::BlueprintBook(data) => data.get_ids(),
            Self::UpgradePlanner(data) => data.get_ids(),
            Self::DeconstructionPlanner(data) => data.get_ids(),
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

    #[instrument(name = "str2bp_data", skip(bp_string))]
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

    #[instrument(name = "bp_data2str", skip(data))]
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&data)?;

        json_to_bp_string(&json)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum VersionExtractor {
    Version { version: Version },
    Other(serde_json::Value),
}

pub fn get_version(bp_string: impl AsRef<str>) -> Result<Version, BlueprintDecodeError> {
    use std::collections::HashMap;

    let json = bp_string_to_json(bp_string.as_ref())?;
    let extractor: HashMap<String, VersionExtractor> = serde_json::from_str(&json)?;

    let version = extractor
        .values()
        .find_map(|ext| {
            let VersionExtractor::Version { version } = ext else {
                return None;
            };
            Some(*version)
        })
        .ok_or(BlueprintDecodeError::Parsing)?;

    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::unwrap_used)]
    fn load_bp(data: &str) -> Data {
        Data::try_from(data).unwrap()
    }

    macro_rules! bp_tests {
        ($prefix:literal, $($name:ident),+) => {
            $(
                #[test]
                fn $name() {
                    load_bp(include_str!(concat!(
                        "../tests/",
                        $prefix,
                        stringify!($name),
                        ".txt"
                    )));
                }
            )+
        };
    }

    mod bp {
        use super::*;

        // #[test]
        // fn train_schedule_temporary_record() {
        //     load_bp(include_str!("../tests/train_schedule_temporary_record.txt"));
        // }

        // #[test]
        // fn comparators_operators_and_invalids() {
        //     load_bp(include_str!(
        //         "../tests/comparators_operators_and_invalids.txt"
        //     ));
        // }

        mod v2 {
            use super::*;

            bp_tests!(
                "2.0/",
                artillery,
                combinators,
                constant_logistic_group,
                elevated_rails,
                flipped_fluidboxes,
                long_train,
                parametrics,
                platform_hub,
                quality_chemplants,
                rail_test_circle,
                test_book,
                train_schedule
            );
        }
    }

    #[allow(clippy::unwrap_used)]
    mod entity {
        use super::*;

        #[test]
        fn selector() {
            let src = r#"{
  "entity_number": 7,
  "name": "selector-combinator",
  "position": {
    "x": 44,
    "y": -26.5
  },
  "direction": 4,
  "control_behavior": {
    "operation": "select",
    "select_max": false,
    "index_signal": {
      "type": "virtual",
      "name": "signal-dot",
      "quality": "legendary"
    }
  }
}"#;

            let sc = serde_json::from_str::<blueprint::Entity>(src).unwrap();
            assert_eq!(src, serde_json::to_string_pretty(&sc).unwrap());

            assert_eq!(sc.name, EntityID::new("selector-combinator"));
            assert_eq!(sc.entity_number, 7);
            assert_eq!(sc.direction, types::Direction::East);
            assert_eq!(sc.position, Position { x: 44.0, y: -26.5 });

            let Some(cb) = sc.control_behavior else {
                panic!("control_behavior is None");
            };

            let Some(scd) = cb.selector_conditions else {
                panic!("selector_conditions is None");
            };

            let SelectorData::Select {
                select_max,
                index_signal,
                index_constant,
            } = scd
            else {
                panic!("selector_conditions is not Select");
            };

            assert!(!select_max);
            assert_eq!(index_constant, None);

            let Some(index_signal) = index_signal else {
                panic!("index_signal is None");
            };

            let SignalID::Virtual { name, quality } = index_signal else {
                panic!("index_signal is not Virtual");
            };

            assert_eq!(name, Some(VirtualSignalID::new("signal-dot")));
            assert_eq!(quality, Some(QualityID::new("legendary")));
        }
    }
}
