use std::collections::{BTreeMap, HashMap};

use mod_util::{mod_info::DependencyVersion, AnyBasic, DependencyList};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use types::{
    ArithmeticOperation, Comparator, Direction, FilterMode, ItemCountType, ItemStackIndex,
    RealOrientation, Vector,
};

use crate::{IndexedVec, NameString};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintData {
    #[serde(flatten)]
    pub snapping: SnapData,

    pub icons: IndexedVec<Icon>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<Tile>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<Schedule>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

impl crate::GetIDs for BlueprintData {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = self.icons.get_ids();

        ids.merge(self.entities.get_ids());

        for tile in &self.tiles {
            ids.tile.insert(tile.name.clone());
        }

        ids.merge(self.schedules.get_ids());

        ids
    }
}

impl BlueprintData {
    #[must_use]
    pub fn has_meta_info(&self) -> bool {
        self.entities
            .iter()
            .any(|e| e.tags.contains_key("bp_meta_info"))
    }

    #[must_use]
    pub fn get_meta_info_mods(&self) -> Option<DependencyList> {
        for e in &self.entities {
            let Some(info) = e.tags.get("bp_meta_info") else {
                continue;
            };

            let AnyBasic::Table(data) = info else {
                continue;
            };

            let Some(mods) = data.get("mods") else {
                continue;
            };

            let AnyBasic::Table(mods) = mods else {
                continue;
            };

            let mut result = HashMap::with_capacity(mods.len());

            for (mod_name, mod_version) in mods {
                let AnyBasic::String(mod_version) = mod_version else {
                    continue;
                };

                let Ok(version) = mod_version.try_into() else {
                    continue;
                };

                result.insert(mod_name.clone(), DependencyVersion::Exact(version));
            }

            return Some(result);
        }

        None
    }
}

pub type Blueprint = crate::CommonData<BlueprintData>;

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
}

impl crate::GetIDs for Icon {
    fn get_ids(&self) -> crate::UsedIDs {
        self.signal.get_ids()
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalID {
    Item { name: Option<String> },
    Fluid { name: Option<String> },
    Virtual { name: Option<String> },
}

impl SignalID {
    #[must_use]
    pub const fn name(&self) -> &Option<String> {
        match self {
            Self::Item { name } | Self::Fluid { name } | Self::Virtual { name } => name,
        }
    }
}

impl crate::GetIDs for SignalID {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        if let Some(name) = self.name() {
            match self {
                Self::Item { .. } => ids.item.insert(name.clone()),
                Self::Fluid { .. } => ids.fluid.insert(name.clone()),
                Self::Virtual { .. } => ids.virtual_signal.insert(name.clone()),
            };
        }

        ids
    }
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
    pub filters: IndexedVec<NameString>,

    pub filter_mode: Option<FilterMode>,
    pub override_stack_size: Option<u8>,
    pub drop_position: Option<Position>,
    pub pickup_position: Option<Position>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_filters: IndexedVec<LogisticFilter>,

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

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub tags: mod_util::TagTable,
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl crate::GetIDs for Entity {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        ids.entity.insert(self.name.clone());

        if let Some(control_behavior) = &self.control_behavior {
            ids.merge(control_behavior.get_ids());
        }

        for item in self.items.keys() {
            ids.item.insert(item.clone());
        }

        if !self.recipe.is_empty() {
            ids.recipe.insert(self.recipe.clone());
        }

        if let Some(inventory) = &self.inventory {
            ids.merge(inventory.get_ids());
        }

        if let Some(infinity_settings) = &self.infinity_settings {
            ids.merge(infinity_settings.get_ids());
        }

        if !self.filter.is_empty() {
            ids.item.insert(self.filter.clone());
        }

        for entry in &self.filters {
            ids.item.insert(entry.name.clone());
        }

        for entry in &self.request_filters {
            ids.item.insert(entry.name.clone());
        }

        if let Some(alert_parameters) = &self.alert_parameters {
            if let Some(signal) = &alert_parameters.icon_signal_id {
                ids.merge(signal.get_ids());
            }
        }

        ids
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

impl SplitterPriority {
    #[must_use]
    pub const fn as_vector(&self) -> Vector {
        match self {
            Self::Left => Vector::Tuple(-0.5, 0.0),
            Self::Right => Vector::Tuple(0.5, 0.0),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: IndexedVec<NameString>,
    pub bar: Option<ItemStackIndex>,
}

impl crate::GetIDs for Inventory {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        for entry in &self.filters {
            ids.item.insert(entry.name.clone());
        }

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schedule {
    pub schedule: Vec<ScheduleRecord>,
    pub locomotives: Vec<EntityNumber>,
}

impl crate::GetIDs for Schedule {
    fn get_ids(&self) -> crate::UsedIDs {
        self.schedule.get_ids()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleRecord {
    pub station: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wait_conditions: Vec<WaitCondition>,
}

impl crate::GetIDs for ScheduleRecord {
    fn get_ids(&self) -> crate::UsedIDs {
        self.wait_conditions.get_ids()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
//#[serde(deny_unknown_fields)] // causes deserialization issues (https://github.com/serde-rs/serde/issues/1358)
pub struct WaitCondition {
    pub compare_type: CompareType,

    #[serde(flatten)]
    pub condition: WaitConditionType,
}

impl crate::GetIDs for WaitCondition {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match &self.condition {
            WaitConditionType::Circuit { condition }
            | WaitConditionType::ItemCount { condition }
            | WaitConditionType::FluidCount { condition } => {
                if let Some(condition) = condition {
                    ids.merge(condition.get_ids());
                }
            }
            _ => {}
        }

        ids
    }
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
        first_signal: Option<SignalID>,
        #[serde(default)]
        constant: i32,
        comparator: Comparator,
    },
}

impl crate::GetIDs for Condition {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::Signals {
                first_signal,
                second_signal,
                ..
            } => {
                if let Some(signal) = first_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = second_signal {
                    ids.merge(signal.get_ids());
                }
            }
            Self::Constant { first_signal, .. } => {
                if let Some(signal) = first_signal {
                    ids.merge(signal.get_ids());
                }
            }
        }

        ids
    }
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
    Switch {
        #[serde(rename = "1")]
        one: ConnectionPoint,

        #[serde(rename = "Cu0", default, skip_serializing_if = "Vec::is_empty")]
        cu0: Vec<ConnectionData>,
        #[serde(rename = "Cu1", default, skip_serializing_if = "Vec::is_empty")]
        cu1: Vec<ConnectionData>,
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

impl ConnectionPoint {
    pub fn transform(&self, map: &mut HashMap<EntityNumber, [bool; 3]>) {
        for r in &self.red {
            map.entry(r.entity_id())
                .and_modify(|[_, x, _]| *x = true)
                .or_insert([false, true, false]);
        }

        for g in &self.green {
            map.entry(g.entity_id())
                .and_modify(|[_, _, x]| *x = true)
                .or_insert([false, false, true]);
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum ConnectionData {
    Connector {
        entity_id: EntityNumber,
        circuit_id: u8,
    },
    Switch {
        entity_id: EntityNumber,
        wire_id: u8,
    },
    NoConnector {
        entity_id: EntityNumber,
    },
}

impl ConnectionData {
    #[must_use]
    pub const fn entity_id(&self) -> EntityNumber {
        match self {
            Self::Connector { entity_id, .. }
            | Self::Switch { entity_id, .. }
            | Self::NoConnector { entity_id } => *entity_id,
        }
    }
}

pub trait ConnectionDataExt {
    fn transform(&self, map: &mut HashMap<EntityNumber, [bool; 3]>);
}

impl ConnectionDataExt for Vec<ConnectionData> {
    fn transform(&self, map: &mut HashMap<EntityNumber, [bool; 3]>) {
        for data in self {
            if let ConnectionData::Switch { entity_id, .. } = data {
                map.entry(*entity_id)
                    .and_modify(|[x, _, _]| *x = true)
                    .or_insert([true, false, false]);
            }
        }
    }
}

pub type ItemRequest = HashMap<String, ItemCountType>;

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
        filters: Option<IndexedVec<InfinityFilter>>,
    },
}

impl crate::GetIDs for InfinitySettings {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::Pipe { name, .. } => {
                if let Some(name) = name {
                    ids.fluid.insert(name.clone());
                }
            }
            Self::Chest { filters, .. } => {
                if let Some(filters) = filters {
                    for entry in filters {
                        ids.item.insert(entry.name.clone());
                    }
                }
            }
        }

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InfinityFilter {
    pub name: String,
    pub count: ItemCountType,
    pub mode: InfinityFilterMode,
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
    pub icon_signal_id: Option<SignalID>, // can be missing if not set
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
    pub filters: IndexedVec<ConstantCombinatorFilter>,
    pub arithmetic_conditions: Option<ArithmeticData>,
    pub decider_conditions: Option<DeciderData>,

    // speakers
    pub circuit_parameters: Option<SpeakerCircuitParameters>,

    // lamps
    pub use_colors: Option<bool>,
}

impl crate::GetIDs for ControlBehavior {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        if let Some(logistic_condition) = &self.logistic_condition {
            ids.merge(logistic_condition.get_ids());
        }

        if let Some(red_output_signal) = &self.red_output_signal {
            ids.merge(red_output_signal.get_ids());
        }

        if let Some(orange_output_signal) = &self.orange_output_signal {
            ids.merge(orange_output_signal.get_ids());
        }

        if let Some(green_output_signal) = &self.green_output_signal {
            ids.merge(green_output_signal.get_ids());
        }

        if let Some(blue_output_signal) = &self.blue_output_signal {
            ids.merge(blue_output_signal.get_ids());
        }

        if let Some(circuit_condition) = &self.circuit_condition {
            ids.merge(circuit_condition.get_ids());
        }

        if let Some(train_stopped_signal) = &self.train_stopped_signal {
            ids.merge(train_stopped_signal.get_ids());
        }

        if let Some(trains_limit_signal) = &self.trains_limit_signal {
            ids.merge(trains_limit_signal.get_ids());
        }

        if let Some(trains_count_signal) = &self.trains_count_signal {
            ids.merge(trains_count_signal.get_ids());
        }

        if let Some(available_logistic_output_signal) = &self.available_logistic_output_signal {
            ids.merge(available_logistic_output_signal.get_ids());
        }

        if let Some(total_logistic_output_signal) = &self.total_logistic_output_signal {
            ids.merge(total_logistic_output_signal.get_ids());
        }

        if let Some(available_construction_output_signal) =
            &self.available_construction_output_signal
        {
            ids.merge(available_construction_output_signal.get_ids());
        }

        if let Some(total_construction_output_signal) = &self.total_construction_output_signal {
            ids.merge(total_construction_output_signal.get_ids());
        }

        if let Some(output_signal) = &self.output_signal {
            ids.merge(output_signal.get_ids());
        }

        if let Some(stack_control_input_signal) = &self.stack_control_input_signal {
            ids.merge(stack_control_input_signal.get_ids());
        }

        ids.merge(self.filters.get_ids());

        if let Some(arithmetic_conditions) = &self.arithmetic_conditions {
            ids.merge(arithmetic_conditions.get_ids());
        }

        if let Some(decider_conditions) = &self.decider_conditions {
            ids.merge(decider_conditions.get_ids());
        }

        ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConstantCombinatorFilter {
    pub signal: SignalID,
    pub count: i32,
}

impl crate::GetIDs for ConstantCombinatorFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        self.signal.get_ids()
    }
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
    pub const fn operation(&self) -> ArithmeticOperation {
        match self {
            Self::SignalSignal { operation, .. }
            | Self::SignalConstant { operation, .. }
            | Self::ConstantSignal { operation, .. }
            | Self::ConstantConstant { operation, .. } => *operation,
        }
    }
}

impl crate::GetIDs for ArithmeticData {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::SignalSignal {
                first_signal,
                second_signal,
                output_signal,
                ..
            } => {
                if let Some(signal) = first_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = second_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = output_signal {
                    ids.merge(signal.get_ids());
                }
            }
            Self::SignalConstant {
                first_signal,
                output_signal,
                ..
            } => {
                if let Some(signal) = first_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = output_signal {
                    ids.merge(signal.get_ids());
                }
            }
            Self::ConstantSignal {
                second_signal,
                output_signal,
                ..
            } => {
                if let Some(signal) = second_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = output_signal {
                    ids.merge(signal.get_ids());
                }
            }
            Self::ConstantConstant { output_signal, .. } => {
                if let Some(signal) = output_signal {
                    ids.merge(signal.get_ids());
                }
            }
        }

        ids
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

        #[serde(default = "default_true", skip_serializing_if = "Clone::clone")]
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
    pub const fn operation(&self) -> Comparator {
        match self {
            Self::Signal { comparator, .. } | Self::Constant { comparator, .. } => *comparator,
        }
    }
}

impl crate::GetIDs for DeciderData {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            Self::Signal {
                first_signal,
                second_signal,
                output_signal,
                ..
            } => {
                if let Some(signal) = first_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = second_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = output_signal {
                    ids.merge(signal.get_ids());
                }
            }
            Self::Constant {
                first_signal,
                output_signal,
                ..
            } => {
                if let Some(signal) = first_signal {
                    ids.merge(signal.get_ids());
                }

                if let Some(signal) = output_signal {
                    ids.merge(signal.get_ids());
                }
            }
        }

        ids
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
    S: serde::Serializer,
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
