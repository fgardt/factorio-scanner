#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(dead_code)]

use std::{collections::HashMap, io::prelude::*};

use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize, Serializer};
use serde_with::skip_serializing_none;

use types::{ItemCountType, ItemStackIndex};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Data {
    BlueprintBook {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        icons: Vec<Icon>,

        blueprints: Vec<BookData>,
        active_index: u16,

        #[serde(flatten)]
        info: Common,
    },
    Blueprint {
        #[serde(default, skip_serializing_if = "String::is_empty")]
        description: String,

        #[serde(flatten)]
        snapping: SnapData,

        icons: Vec<Icon>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        entities: Vec<Entity>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tiles: Vec<Tile>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        schedules: Vec<Schedule>,

        #[serde(flatten)]
        info: Common,
    },
}

impl Data {
    #[must_use]
    pub const fn get_info(&self) -> &Common {
        match self {
            Self::Blueprint { info, .. } | Self::BlueprintBook { info, .. } => info,
        }
    }

    fn normalize_positions(&mut self) {
        match self {
            Self::BlueprintBook { blueprints, .. } => {
                for blueprint in blueprints {
                    blueprint.data.normalize_positions();
                }
            }
            Self::Blueprint {
                entities, tiles, ..
            } => {
                let mut min_x = f32::MAX;
                let mut min_y = f32::MAX;
                let mut max_x = f32::MIN;
                let mut max_y = f32::MIN;

                for entity in &*entities {
                    min_x = min_x.min(entity.position.x);
                    min_y = min_y.min(entity.position.y);
                    max_x = max_x.max(entity.position.x);
                    max_y = max_y.max(entity.position.y);
                }

                for tile in &*tiles {
                    min_x = min_x.min(tile.position.x);
                    min_y = min_y.min(tile.position.y);
                    max_x = max_x.max(tile.position.x);
                    max_y = max_y.max(tile.position.y);
                }

                let width = ((max_x - min_x) / 2.0).round();
                let height = ((max_y - min_y) / 2.0).round();
                let offset_x = (min_x + width).round();
                let offset_y = (min_y + height).round();

                for entity in entities {
                    entity.position.x -= offset_x;
                    entity.position.y -= offset_y;
                }

                for tile in tiles {
                    tile.position.x -= offset_x;
                    tile.position.y -= offset_y;
                }
            }
        }
    }
}

// TODO: properly propagate/bubble errors up for better handling

impl TryFrom<&str> for Data {
    type Error = &'static str;

    fn try_from(bp_string: &str) -> Result<Self, Self::Error> {
        if bp_string.len() < 2 {
            return Err("Blueprint string must be at least 2 characters long.");
        }

        let mut chars = bp_string.chars();

        match chars.next() {
            Some(first) => {
                if first != '0' {
                    return Err("Unsupported blueprint version.");
                }
            }
            None => return Err("Error parsing blueprint string."),
        }

        let Ok(compressed) = general_purpose::STANDARD.decode(chars.as_str()) else {
            return Err("Error decoding blueprint string.");
        };

        let mut deflate = ZlibDecoder::new(compressed.as_slice());
        let mut uncompressed = String::new();

        if deflate.read_to_string(&mut uncompressed).is_err() {
            return Err("Error decompressing blueprint string.");
        }

        let mut data: Self = serde_json::from_str(&uncompressed).unwrap(); //.map_or(Err("Error deserializing blueprint."), Ok)?;

        data.normalize_positions();

        Ok(data)
    }
}

impl TryFrom<String> for Data {
    type Error = &'static str;

    fn try_from(bp_string: String) -> Result<Self, Self::Error> {
        Self::try_from(bp_string.as_str())
    }
}

impl TryFrom<Data> for String {
    type Error = &'static str;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let Ok(uncompressed) = serde_json::to_string(&data) else {
            return Err("Error serializing blueprint.");
        };

        let mut deflate = ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
        match deflate.write_all(uncompressed.as_bytes()) {
            Ok(()) => (),
            Err(_) => return Err("Error compressing blueprint."),
        };

        let compressed = deflate.finish().unwrap();

        let mut encoded = general_purpose::STANDARD.encode(compressed);

        encoded.insert(0, '0');

        Ok(encoded)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BookData {
    #[serde(flatten)]
    pub data: Data,
    pub index: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Common {
    pub item: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,

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
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub entity_number: EntityNumber,
    pub name: String,
    pub position: Position,
    pub direction: Option<u8>,
    pub orientation: Option<f32>,

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
    pub tags: TagTable,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UndergroundType {
    Input,
    Output,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitterPriority {
    Left,
    Right,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterMode {
    Whitelist,
    Blacklist,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Comparator {
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "≤", alias = "<=")]
    LessOrEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = "≥", alias = ">=")]
    GreaterOrEqual,
    #[serde(rename = "=")]
    Equal,
    #[serde(rename = "≠", alias = "!=")]
    NotEqual,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CompareType {
    And,
    Or,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    pub name: String,
    pub position: Position,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(serialize_with = "shorter_floats")]
    pub x: f32,

    #[serde(serialize_with = "shorter_floats")]
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Connection {
    Double {
        #[serde(rename = "1")]
        one: ConnectionPoint,

        #[serde(rename = "2")]
        two: ConnectionPoint,
    },
    Single {
        #[serde(rename = "1")]
        one: ConnectionPoint,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionPoint {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub red: Vec<ConnectionData>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub green: Vec<ConnectionData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ItemFilter {
    pub name: String,
    pub index: ItemStackIndex,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InfinitySettings {
    pub remove_unfiltered_items: bool,
    pub filters: Option<Vec<InfinityFilter>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InfinityFilter {
    pub name: String,
    pub count: ItemCountType,
    pub mode: InfinityFilterMode,
    pub index: ItemStackIndex,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum InfinityFilterMode {
    AtLeast,
    AtMost,
    Exactly,
    Remove,
    Add,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LogisticFilter {
    pub name: String,
    pub index: ItemStackIndex,
    pub count: ItemCountType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpeakerParameter {
    pub playback_volume: f32,
    pub playback_globally: bool,
    pub allow_polyphony: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpeakerAlertParameter {
    pub show_alert: bool,
    pub show_on_map: bool,
    pub icon_signal_id: SignalID,
    pub alert_message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstantCombinatorFilter {
    pub signal: SignalID,
    pub count: i32,
    pub index: ItemStackIndex,
}

// https://lua-api.factorio.com/latest/concepts.html#ArithmeticCombinatorParameters
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArithmeticOperation {
    #[serde(rename = "*")]
    Multiply,

    #[serde(rename = "/")]
    Divide,

    #[serde(rename = "+")]
    Add,

    #[serde(rename = "-")]
    Subtract,

    #[serde(rename = "%")]
    Modulo,

    #[serde(rename = "^")]
    Power,

    #[serde(rename = "<<")]
    LeftShift,

    #[serde(rename = ">>")]
    RightShift,

    #[serde(rename = "AND")]
    BitwiseAnd,

    #[serde(rename = "OR")]
    BitwiseOr,

    #[serde(rename = "XOR")]
    BitwiseXor,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
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

// https://lua-api.factorio.com/latest/concepts.html#DeciderCombinatorParameters
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpeakerCircuitParameters {
    //#[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub signal_value_is_pitch: bool,
    pub instrument_id: u8,
    pub note_id: u8,
}

// https://lua-api.factorio.com/latest/concepts.html#Tags
pub type TagTable = HashMap<String, AnyBasic>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum AnyBasic {
    String(String),
    Bool(bool),
    Number(f64),
    Table(TagTable),
    Array(Vec<AnyBasic>),
}

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
