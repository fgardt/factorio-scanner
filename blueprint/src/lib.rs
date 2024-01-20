#![allow(dead_code)]

use std::{collections::HashMap, convert::Infallible, error::Error, io::prelude::*};

use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize, Serializer};
use serde_with::skip_serializing_none;

use types::{
    ArithmeticOperation, Comparator, Direction, FilterMode, ItemCountType, ItemStackIndex,
    RealOrientation,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Data {
    BlueprintBook(Book),
    Blueprint(Box<Blueprint>),
}

impl Data {
    #[must_use]
    pub const fn get_info(&self) -> &Common {
        match self {
            Self::Blueprint(data) => &data.info,
            Self::BlueprintBook(data) => &data.info,
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
            Self::Blueprint(_) => None,
            Self::BlueprintBook(data) => Some(data),
        }
    }

    pub fn as_book_mut(&mut self) -> Option<&mut Book> {
        match self {
            Self::Blueprint(_) => None,
            Self::BlueprintBook(data) => Some(data),
        }
    }

    #[must_use]
    pub fn as_blueprint(&self) -> Option<&Blueprint> {
        match self {
            Self::Blueprint(data) => Some(data),
            Self::BlueprintBook(book) => {
                if book.blueprints.is_empty() {
                    None
                } else {
                    book.blueprints
                        .get(book.active_index as usize)
                        .and_then(|entry| entry.data.as_blueprint())
                }
            }
        }
    }

    pub fn as_blueprint_mut(&mut self) -> Option<&mut Blueprint> {
        match self {
            Self::Blueprint(data) => Some(data),
            Self::BlueprintBook(_) => None,
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
        }
    }
}

// TODO: properly propagate/bubble errors up for better handling

#[derive(Debug, Clone)]
pub enum BlueprintError {
    MinSize,
    UnsupportedVersion(char),
    Parsing,
    Decoding(String),
    Decompressing(String),
    Deserializing(String),
    Serializing(String),
    Compressing(String),
}

impl Error for BlueprintError {}

impl std::fmt::Display for BlueprintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MinSize => write!(f, "blueprint string must be at least 2 characters long"),
            Self::UnsupportedVersion(v) => write!(f, "unsupported blueprint version: {v}"),
            Self::Parsing => write!(f, "blueprint string parsing failed"),
            Self::Decoding(info) => write!(f, "blueprint string decoding failed:\n{info}"),
            Self::Decompressing(info) => {
                write!(f, "blueprint string decompression failed:\n{info}")
            }
            Self::Deserializing(info) => {
                write!(f, "blueprint string deserialization failed:\n{info}")
            }
            Self::Serializing(info) => write!(f, "blueprint string serialization failed:\n{info}"),
            Self::Compressing(info) => write!(f, "blueprint string compression failed:\n{info}"),
        }
    }
}

impl From<Result<Infallible, Self>> for BlueprintError {
    fn from(res: Result<Infallible, Self>) -> Self {
        match res {
            Ok(_) => unreachable!(),
            Err(err) => err,
        }
    }
}

pub fn bp_string_to_json(bp_string: &str) -> Result<String, BlueprintError> {
    if bp_string.len() < 2 {
        return Err(BlueprintError::MinSize);
    }

    let mut chars = bp_string.chars();

    match chars.next() {
        Some(first) => {
            if first != '0' {
                return Err(BlueprintError::UnsupportedVersion(first));
            }
        }
        None => return Err(BlueprintError::Parsing),
    }

    let compressed = general_purpose::STANDARD
        .decode(chars.as_str())
        .map_err(|err| Err(BlueprintError::Decoding(err.to_string())))?;

    let mut deflate = ZlibDecoder::new(compressed.as_slice());
    let mut uncompressed = String::new();

    deflate
        .read_to_string(&mut uncompressed)
        .map_err(|err| Err(BlueprintError::Decompressing(err.to_string())))?;

    Ok(uncompressed)
}

pub fn json_to_bp_string(json: &str) -> Result<String, BlueprintError> {
    let mut deflate = ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    deflate
        .write_all(json.as_bytes())
        .map_err(|err| Err(BlueprintError::Compressing(err.to_string())))?;

    let compressed = deflate
        .finish()
        .map_err(|err| Err(BlueprintError::Compressing(err.to_string())))?;

    let mut encoded = general_purpose::STANDARD.encode(compressed);

    encoded.insert(0, '0');

    Ok(encoded)
}

impl TryFrom<&str> for Data {
    type Error = BlueprintError;

    fn try_from(bp_string: &str) -> Result<Self, Self::Error> {
        let json = bp_string_to_json(bp_string)?;
        let mut data: Self = serde_json::from_str(&json)
            .map_err(|err| Err(BlueprintError::Deserializing(err.to_string())))?;

        data.normalize_positions();
        data.ensure_ordering();

        Ok(data)
    }
}

impl TryFrom<String> for Data {
    type Error = BlueprintError;

    fn try_from(bp_string: String) -> Result<Self, Self::Error> {
        Self::try_from(bp_string.as_str())
    }
}

impl TryFrom<Data> for String {
    type Error = BlueprintError;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&data)
            .map_err(|err| Err(BlueprintError::Serializing(err.to_string())))?;

        json_to_bp_string(&json)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Book {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<Icon>,

    pub blueprints: Vec<BookEntry>,
    pub active_index: u16,

    #[serde(flatten)]
    pub info: Common,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BookEntry {
    #[serde(flatten)]
    pub data: Data,
    pub index: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Blueprint {
    #[serde(flatten)]
    pub snapping: SnapData,

    pub icons: Vec<Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<Tile>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<Schedule>,

    #[serde(flatten)]
    pub info: Common,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Common {
    pub item: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    pub version: u64, // see https://wiki.factorio.com/Version_string_format
}

impl Common {
    #[must_use]
    pub fn version_string(&self) -> String {
        let major = self.version >> (64 - 2 * 8);
        let minor = self.version >> (64 - 4 * 8) & 0xFF;
        let patch = self.version >> (64 - 6 * 8) & 0xFF;
        //let dev = self.version & 0xFF;

        format!("{major}.{minor}.{patch}") //-{dev}")
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SnapData {
    pub snap_to_grid: Option<Position>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub absolute_snapping: bool,

    pub position_relative_to_grid: Option<Position>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    pub signal: SignalID,
    pub index: u8,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalID {
    Item { name: Option<String> },
    Fluid { name: Option<String> },
    Virtual { name: Option<String> },
}

pub type EntityNumber = u64;
pub type GraphicsVariation = u8;

// todo: reduce optionals count by skipping serialization of defaults?
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub entity_number: EntityNumber,
    pub name: String,
    pub position: Position,

    #[serde(default, skip_serializing_if = "Direction::is_default")]
    pub direction: Direction,

    pub orientation: Option<RealOrientation>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub neighbours: Vec<EntityNumber>,

    pub control_behavior: Option<ControlBehavior>,
    pub connections: Option<Connection>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub items: ItemRequest,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub recipe: String,

    pub bar: Option<ItemStackIndex>,
    pub inventory: Option<Inventory>,
    pub infinity_settings: Option<InfinitySettings>,

    #[serde(rename = "type")]
    pub type_: Option<UndergroundType>,
    pub belt_link: Option<u32>,
    pub link_id: Option<u32>,

    pub input_priority: Option<SplitterPriority>,
    pub output_priority: Option<SplitterPriority>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub filter: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<ItemFilter>,

    pub filter_mode: Option<FilterMode>,
    pub override_stack_size: Option<u8>,
    pub drop_position: Option<Position>,
    pub pickup_position: Option<Position>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_filters: Vec<LogisticFilter>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub request_from_buffers: bool,

    pub parameters: Option<SpeakerParameter>,
    pub alert_parameters: Option<SpeakerAlertParameter>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub auto_launch: bool,

    pub variation: Option<GraphicsVariation>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub station: String,

    pub color: Option<Color>,

    pub manual_trains_limit: Option<u32>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub switch_state: bool,

    // electric energy interface
    pub buffer_size: Option<f64>,
    pub power_production: Option<f64>,
    pub power_usage: Option<f64>,

    // heat interface
    pub temperature: Option<f64>,
    pub mode: Option<String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: mod_util::TagTable,
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UndergroundType {
    Input,
    Output,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SplitterPriority {
    Left,
    Right,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    pub filters: Vec<ItemFilter>,
    pub bar: Option<ItemStackIndex>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schedule {
    pub schedule: Vec<ScheduleRecord>,
    pub locomotives: Vec<EntityNumber>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleRecord {
    pub station: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wait_conditions: Vec<WaitCondition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
//#[serde(deny_unknown_fields)] // causes deserialization issues (https://github.com/serde-rs/serde/issues/1358)
pub struct WaitCondition {
    pub compare_type: CompareType,

    #[serde(flatten)]
    pub condition: WaitConditionType,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WaitConditionType {
    Full,
    Empty,
    RobotsInactive,
    PassengerPresent,
    PassengerNotPresent,
    Time { ticks: u32 },
    Inactivity { ticks: u32 },
    Circuit { condition: Option<Condition> },
    ItemCount { condition: Option<Condition> },
    FluidCount { condition: Option<Condition> },
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum Condition {
    Signals {
        first_signal: Option<SignalID>,
        second_signal: Option<SignalID>,
        comparator: Comparator,
    },
    Constant {
        first_signal: SignalID,
        #[serde(default)]
        constant: i32,
        comparator: Comparator,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompareType {
    And,
    Or,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    pub name: String,
    pub position: Position,
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(serialize_with = "shorter_floats")]
    pub x: f32,

    #[serde(serialize_with = "shorter_floats")]
    pub y: f32,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y).map_or_else(
            || self.x.partial_cmp(&other.x),
            |res| match res {
                std::cmp::Ordering::Equal => self.x.partial_cmp(&other.x),
                std::cmp::Ordering::Less | std::cmp::Ordering::Greater => Some(res),
            },
        )
    }
}

impl From<Position> for types::MapPosition {
    fn from(value: Position) -> Self {
        Self::XY {
            x: f64::from(value.x),
            y: f64::from(value.y),
        }
    }
}

impl From<&Position> for types::MapPosition {
    fn from(value: &Position) -> Self {
        Self::XY {
            x: f64::from(value.x),
            y: f64::from(value.y),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum Connection {
    Double {
        #[serde(rename = "1")]
        one: ConnectionPoint,

        #[serde(rename = "2")]
        two: ConnectionPoint,
    },
    SingleOne {
        #[serde(rename = "1")]
        one: ConnectionPoint,
    },
    SingleTwo {
        #[serde(rename = "2")]
        two: ConnectionPoint,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConnectionPoint {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub red: Vec<ConnectionData>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub green: Vec<ConnectionData>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum ConnectionData {
    Connector {
        entity_id: EntityNumber,
        circuit_id: u8,
    },
    NoConnector {
        entity_id: EntityNumber,
    },
}

pub type ItemRequest = HashMap<String, ItemCountType>;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ItemFilter {
    pub name: String,
    pub index: ItemStackIndex,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields, untagged)]
pub enum InfinitySettings {
    Pipe {
        name: Option<String>,             // infinity pipes?
        percentage: Option<f64>,          // infinity pipes?
        temperature: Option<f64>,         // infinity pipes?
        mode: Option<InfinityFilterMode>, // infinity pipes?
    },
    Chest {
        remove_unfiltered_items: bool,
        filters: Option<Vec<InfinityFilter>>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InfinityFilter {
    pub name: String,
    pub count: ItemCountType,
    pub mode: InfinityFilterMode,
    pub index: ItemStackIndex,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InfinityFilterMode {
    AtLeast,
    AtMost,
    Exactly,
    Remove,
    Add,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LogisticFilter {
    pub name: String,
    pub index: ItemStackIndex,
    pub count: ItemCountType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SpeakerParameter {
    pub playback_volume: f32,
    pub playback_globally: bool,
    pub allow_polyphony: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SpeakerAlertParameter {
    pub show_alert: bool,
    pub show_on_map: bool,
    pub icon_signal_id: SignalID,
    pub alert_message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl From<&Color> for types::Color {
    fn from(value: &Color) -> Self {
        Self::RGBA(value.r, value.g, value.b, value.a)
    }
}

#[allow(clippy::struct_excessive_bools)]
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ControlBehavior {
    pub logistic_condition: Option<Condition>,
    pub connect_to_logistic_network: Option<bool>,

    // rail/chain signals
    pub circuit_close_signal: Option<bool>,
    pub circuit_read_signal: Option<bool>,
    pub red_output_signal: Option<SignalID>,
    pub orange_output_signal: Option<SignalID>,
    pub green_output_signal: Option<SignalID>,
    pub blue_output_signal: Option<SignalID>,

    pub circuit_condition: Option<Condition>,
    pub circuit_enable_disable: Option<bool>,

    // train stops
    pub send_to_train: Option<bool>,
    pub read_from_train: Option<bool>,

    pub read_stopped_train: Option<bool>,
    pub train_stopped_signal: Option<SignalID>,

    pub set_trains_limit: Option<bool>,
    pub trains_limit_signal: Option<SignalID>,

    pub read_trains_count: Option<bool>,
    pub trains_count_signal: Option<SignalID>,

    // roboports
    pub read_logistics: Option<bool>,
    pub read_robot_stats: Option<bool>,
    pub available_logistic_output_signal: Option<SignalID>,
    pub total_logistic_output_signal: Option<SignalID>,
    pub available_construction_output_signal: Option<SignalID>,
    pub total_construction_output_signal: Option<SignalID>,

    // walls
    pub circuit_open_gate: Option<bool>,
    pub circuit_read_sensor: Option<bool>,
    pub output_signal: Option<SignalID>,

    // belts
    pub circuit_read_hand_contents: Option<bool>,
    pub circuit_contents_read_mode: Option<u8>,

    // inserters
    pub circuit_set_stack_size: Option<bool>,
    pub stack_control_input_signal: Option<SignalID>,
    pub circuit_mode_of_operation: Option<u8>,
    pub circuit_hand_read_mode: Option<u8>,

    // miners
    pub circuit_read_resources: Option<bool>,
    pub circuit_resource_read_mode: Option<u8>,

    // combinators
    pub is_on: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<ConstantCombinatorFilter>,
    pub arithmetic_conditions: Option<ArithmeticData>,
    pub decider_conditions: Option<DeciderData>,

    // speakers
    pub circuit_parameters: Option<SpeakerCircuitParameters>,

    // lamps
    pub use_colors: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConstantCombinatorFilter {
    pub signal: SignalID,
    pub count: i32,
    pub index: ItemStackIndex,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum ArithmeticData {
    SignalSignal {
        first_signal: Option<SignalID>,
        second_signal: Option<SignalID>,
        operation: ArithmeticOperation,
        output_signal: Option<SignalID>,
    },
    SignalConstant {
        first_signal: Option<SignalID>,
        #[serde(default)]
        second_constant: i32,
        operation: ArithmeticOperation,
        output_signal: Option<SignalID>,
    },
    ConstantSignal {
        #[serde(default)]
        first_constant: i32,
        second_signal: Option<SignalID>,
        operation: ArithmeticOperation,
        output_signal: Option<SignalID>,
    },
    ConstantConstant {
        #[serde(default)]
        first_constant: i32,
        #[serde(default)]
        second_constant: i32,
        operation: ArithmeticOperation,
        output_signal: Option<SignalID>,
    },
}

impl ArithmeticData {
    #[must_use]
    pub fn operation(&self) -> ArithmeticOperation {
        match self {
            Self::SignalSignal { operation, .. }
            | Self::SignalConstant { operation, .. }
            | Self::ConstantSignal { operation, .. }
            | Self::ConstantConstant { operation, .. } => operation.clone(),
        }
    }
}

// https://lua-api.factorio.com/latest/concepts.html#DeciderCombinatorParameters
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum DeciderData {
    Signal {
        first_signal: Option<SignalID>,
        second_signal: Option<SignalID>,

        comparator: Comparator,
        output_signal: Option<SignalID>,

        #[serde(skip_serializing_if = "std::ops::Not::not")]
        copy_count_from_input: bool,
    },
    Constant {
        first_signal: Option<SignalID>,

        #[serde(default)]
        constant: i32,

        comparator: Comparator,
        output_signal: Option<SignalID>,

        #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
        copy_count_from_input: bool,
    },
}

impl DeciderData {
    #[must_use]
    pub fn operation(&self) -> Comparator {
        match self {
            Self::Signal { comparator, .. } | Self::Constant { comparator, .. } => {
                comparator.clone()
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SpeakerCircuitParameters {
    //#[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub signal_value_is_pitch: bool,
    pub instrument_id: u8,
    pub note_id: u8,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn shorter_floats<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // if x.rem_euclid(1.0) == 0.0 {
    //     s.serialize_i32(x.into())
    // } else {
    //     s.serialize_f32(*x)
    // }

    // serialize as integer if possible
    if x.rem_euclid(1.0) == 0.0 {
        #[allow(clippy::cast_possible_truncation)]
        s.serialize_i32(*x as i32)
    } else {
        s.serialize_f32(*x)
    }
}

const fn default_true() -> bool {
    true
}
