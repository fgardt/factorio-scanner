use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use types::{
    AsteroidChunkID, Direction, EntityID, FilterMode, FluidID, ItemStackIndex, QualityID,
    RealOrientation, RecipeID, Vector,
};

mod control_behavior;
mod inventory;

pub use control_behavior::*;
pub use inventory::*;

use crate::{
    Color, IndexedVec, NameString, Position, SignalID, blueprint::logistics::LogisticSections,
};

pub type EntityNumber = u32;

/// [`BlueprintEntity`](https://lua-api.factorio.com/latest/concepts/BlueprintEntity.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
//#[serde(deny_unknown_fields)] // causes deserialization issues (https://github.com/serde-rs/serde/issues/1358)
pub struct Entity {
    pub entity_number: EntityNumber,
    pub name: EntityID,
    pub position: Position,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub direction: Direction,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mirror: bool,
    #[serde(
        default = "QualityID::normal",
        skip_serializing_if = "QualityID::is_normal"
    )]
    pub quality: QualityID,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<InsertPlan>,

    #[serde(default, skip_serializing_if = "mod_util::TagTable::is_empty")]
    pub tags: mod_util::TagTable, // TODO: move TagTable / AnyBasic to types

    // pub wires -> handled outside of entity
    pub burner_fuel_inventory: Option<InventoryWithFilters>,

    #[serde(flatten, default, skip_serializing_if = "helper::is_default")]
    pub extra_data: EntityExtraData,
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
        ids.quality.insert(self.quality.clone());
        ids.merge(self.items.get_ids());
        ids.merge(self.burner_fuel_inventory.get_ids());

        // TODO: get_ids() for extra_data

        ids
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EntityExtraData {
    #[default]
    None,
    Accumulator {
        control_behavior: AccumulatorControlBehavior,
    },
    AgriculturalTower {
        control_behavior: Box<AgriculturalTowerControlBehavior>,
    },
    ArtillertyTurret {
        #[serde(default, skip_serializing_if = "helper::is_default")]
        artillery_auto_targeting: bool,

        control_behavior: Option<Box<ArtilleryTurretControlBehavior>>,
    },
    ArtilleryWagon {
        #[serde(default, skip_serializing_if = "helper::is_default")]
        artillery_auto_targeting: bool,
        color: Option<Color>,

        #[serde(flatten)]
        grid_data: GridData,
        orientation: RealOrientation,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        copy_color_from_train_stop: bool,
    },
    AssemblingMachine {
        recipe: Option<RecipeID>,
        #[serde(
            default = "QualityID::normal",
            skip_serializing_if = "QualityID::is_normal"
        )]
        recipe_quality: QualityID,

        control_behavior: Option<Box<AssemblingMachineControlBehavior>>,
    },
    AsteroidCollector {
        // TODO: figure out what this is???
        // #[serde(rename = "result-inventory")]
        // result_inventory: BlueprintInventory,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "chunk-filter"
        )]
        chunk_filter: IndexedVec<NameString<AsteroidChunkID>>,

        control_behavior: Option<Box<AsteroidCollectorControlBehavior>>,
    },
    Car {
        request_filters: Option<LogisticSections>,

        #[serde(flatten)]
        grid_data: GridData,

        trunk_inventory: Option<InventoryWithFilters>,
        ammo_inventory: Option<InventoryWithFilters>,
        driver_is_main_gunner: Option<bool>,
        selected_gun_index: Option<ItemStackIndex>,

        orientation: RealOrientation,
    },
    CargoLandingPad {
        bar: Option<ItemStackIndex>,
        request_filters: Option<LogisticSections>,

        control_behavior: Option<CargoLandingPadControlBehavior>,
    },
    CargoWagon {
        color: Option<Color>,

        #[serde(flatten)]
        grid_data: GridData,
        orientation: RealOrientation,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        copy_color_from_train_stop: bool,
        inventory: Option<InventoryWithFilters>,
    },
    Combinator(CombinatorData),
    Container {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        filters: IndexedVec<ItemFilter>,
        bar: Option<ItemStackIndex>,

        control_behavior: Option<ContainerControlBehavior>,
    },
    DisplayPanel {
        text: Option<String>,
        icon: Option<SignalID>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        always_show: bool,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        show_in_chart: bool,

        control_behavior: Option<DisplayPanelControlBehavior>,
    },
    ElectricEnergyInterface {
        power_production: Option<f64>,
        power_usage: Option<f64>,
        buffer_size: f64,
    },
    FluidWagon {
        color: Option<Color>,

        #[serde(flatten)]
        grid_data: GridData,
        orientation: RealOrientation,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        copy_color_from_train_stop: bool,
    },
    Furnace {
        control_behavior: Box<FurnaceControlBehavior>,
    },
    HeatInterface {
        temperature: Option<f64>,
        mode: Option<HeatSettingMode>,
    },
    InfinityCargoWagon {
        color: Option<Color>,

        #[serde(flatten)]
        grid_data: GridData,
        orientation: RealOrientation,

        #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
        copy_color_from_train_stop: bool,
        inventory: Option<InventoryWithFilters>,
        infinity_settings: Option<InfinityInventorySettings>,
    },
    InfinityContainer {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        filters: IndexedVec<ItemFilter>,
        bar: Option<ItemStackIndex>,
        request_filters: Option<LogisticSections>,
        infinity_settings: Option<InfinityInventorySettings>,

        control_behavior: Option<ContainerControlBehavior>,
    },
    InfinityPipe {
        infinity_settings: InfinityPipeFilter,
    },
    Inserter {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        filters: IndexedVec<ItemFilter>,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        filter_mode: FilterMode,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        use_filters: bool,

        override_stack_size: Option<u8>,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        spoil_priority: SpoilPriority,

        drop_position: Option<Vector>,
        pickup_position: Option<Vector>,

        control_behavior: Option<Box<InserterControlBehavior>>,
    },
    Lamp {
        color: Option<Color>,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        always_on: bool,

        control_behavior: Option<Box<LampControlBehavior>>,
    },
    LaneSplitter {
        filter: Option<ItemFilter>,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        input_priority: SplitterPriority,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        output_priority: SplitterPriority,
    },
    LinkedBelt {
        #[serde(rename = "type")]
        kind: BeltConnectionType,
        belt_link: Option<u32>,
    },
    LinkedContainer {
        link_id: u32,
    },
    // Loader & Loader-1x1
    Loader {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        filters: Vec<ItemFilter>,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        filter_mode: FilterMode,
        #[serde(rename = "type")]
        kind: BeltConnectionType,
        belt_stack_size_override: Option<u8>,
    },
    Locomotive {
        // schedule -> handled outside of entity
        color: Option<Color>,

        #[serde(flatten)]
        grid_data: GridData,

        orientation: RealOrientation,
        copy_color_from_train_stop: Option<bool>,
    },
    LogisticContainer {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        filters: IndexedVec<ItemFilter>,
        bar: Option<ItemStackIndex>,
        request_filters: Option<LogisticSections>,

        control_behavior: Option<Box<LogisticContainerControlBehavior>>,
    },
    MiningDrill {
        filter: Option<MiningDrillFilter>,

        control_behavior: Option<Box<MiningDrillControlBehavior>>,
    },
    PowerSwitch {
        switch_state: Option<bool>,

        control_behavior: Option<Box<CommonControlBehavior>>,
    },
    ProgrammableSpeaker {
        parameters: SpeakerParameters,
        alert_parameters: SpeakerAlertParameters,

        control_behavior: Option<Box<ProgrammableSpeakerControlBehavior>>,
    },
    ProxyContainer {
        // ProxyContainerCB == ContainerCB
        control_behavior: ContainerControlBehavior,
    },
    Pump {
        fluid_filter: Option<FluidID>,

        control_behavior: Option<Box<PumpControlBehavior>>,
    },
    // ChainSignal & Signal
    RailSignal {
        control_behavior: Box<RailSignalControlBehavior>,
    },
    Reactor {
        control_behavior: Box<ReactorControlBehavior>,
    },
    Roboport {
        request_filters: Option<LogisticSections>,

        control_behavior: Option<Box<RoboportControlBehavior>>,
    },
    RocketSilo {
        recipe: Option<RecipeID>,
        #[serde(
            default = "QualityID::normal",
            skip_serializing_if = "QualityID::is_normal"
        )]
        recipe_quality: QualityID,

        use_transitional_requests: Option<bool>,
        launch_to_orbit_automatically: Option<bool>,

        control_behavior: Option<Box<RocketSiloControlBehavior>>,
    },
    SpacePlatformHub {
        bar: Option<ItemStackIndex>,
        request_filters: Option<LogisticSections>,
        request_missing_construction_materials: bool,

        control_behavior: Option<Box<SpacePlatformHubControlBehavior>>,
    },
    Splitter {
        filter: Option<ItemFilter>,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        input_priority: SplitterPriority,
        #[serde(default, skip_serializing_if = "helper::is_default")]
        output_priority: SplitterPriority,

        control_behavior: Option<Box<SplitterControlBehavior>>,
    },
    SpiderVehicle {
        request_filters: Option<LogisticSections>,

        #[serde(flatten)]
        grid_data: GridData,

        trunk_inventory: Option<InventoryWithFilters>,
        ammo_inventory: Option<InventoryWithFilters>,
        driver_is_main_gunner: Option<bool>,
        automatic_targeting_parameters: Option<VehicleAutomaticTargetingParameters>,
        selected_gun_index: Option<ItemStackIndex>,

        color: Option<Color>,
        label: Option<String>,
    },
    StorageTank {
        control_behavior: Box<StorageTankControlBehavior>,
    },
    TrainStop {
        station: String,
        color: Option<Color>,
        manual_trains_limit: Option<u32>,
        priority: Option<u8>,

        control_behavior: Option<Box<TrainStopControlBehavior>>,
    },
    TransportBelt {
        control_behavior: Box<TransportBeltControlBehavior>,
    },
    // AmmoTurret, ElectricTurret, FluidTurret, Turret
    Turret {
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "priority-list"
        )]
        priority_list: IndexedVec<NameString<String>>,
        #[serde(
            default,
            skip_serializing_if = "helper::is_default",
            rename = "ignore-unprioritised"
        )]
        ignore_unprioritised: bool,
        control_behavior: Option<Box<TurretControlBehavior>>,
    },
    UndergroundBelt {
        #[serde(rename = "type")]
        kind: BeltConnectionType,
    },
    Valve {
        valve_threshold_override: f32,
    },
    Wall {
        control_behavior: Box<WallControlBehavior>,
    },
}

/// Inner data for `arithmetic-combinator`, `constant-combinator`, `decider-combinator`, and `selector-combinator`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CombinatorData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub player_description: String,

    pub control_behavior: Option<Box<CombinatorControlBehavior>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GridData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub enable_logistics_while_moving: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub grid: Vec<BlueprintEquipment>,
}

/// [HeatSettingMode](https://lua-api.factorio.com/latest/concepts/HeatSettingMode.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HeatSettingMode {
    AtLeast,
    AtMost,
    Exactly,
    Add,
    Remove,
}

/// [`InfinityPipeFilter`](https://lua-api.factorio.com/latest/concepts/InfinityPipeFilter.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InfinityPipeFilter {
    pub name: FluidID,
    pub percentage: Option<f64>,
    pub temperature: Option<f64>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mode: InfinityPipeFilterMode,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InfinityPipeFilterMode {
    #[default]
    AtLeast,
    AtMost,
    Exactly,
    Add,
    Remove,
}

/// [`SpoilPriority`](https://lua-api.factorio.com/latest/concepts/SpoilPriority.html)
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SpoilPriority {
    #[default]
    None,
    SpoiledFirst,
    FreshFirst,
}

/// [`SplitterPriority`](https://lua-api.factorio.com/latest/concepts/SplitterPriority.html)
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SplitterPriority {
    Left,
    #[default]
    None,
    Right,
}

impl SplitterPriority {
    #[must_use]
    pub const fn as_vector(&self) -> Option<Vector> {
        match self {
            Self::None => None,
            Self::Left => Some(Vector::Tuple(-0.5, 0.0)),
            Self::Right => Some(Vector::Tuple(0.5, 0.0)),
        }
    }
}

/// [`BeltConnectionType`](https://lua-api.factorio.com/latest/concepts/BeltConnectionType.html)
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BeltConnectionType {
    Input,
    Output,
}

/// [`BlueprintMiningDrillFilter`](https://lua-api.factorio.com/latest/concepts/BlueprintMiningDrillFilter.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MiningDrillFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: IndexedVec<NameString<String>>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mode: FilterMode,
}

/// [`ProgrammableSpeakerParameters`](https://lua-api.factorio.com/latest/concepts/ProgrammableSpeakerParameters.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SpeakerParameters {
    pub playback_volume: f32,
    pub playback_mode: SpeakerPlaybackMode,
    pub allow_polyphony: bool,
    pub volume_controlled_by_signal: bool,
    pub volume_signal_id: Option<SignalID>,
}

/// [`ProgrammableSpeakerPlaybackMode`](https://lua-api.factorio.com/latest/concepts/ProgrammableSpeakerPlaybackMode.html)
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpeakerPlaybackMode {
    Local,
    Surface,
    Global,
}

/// [`ProgrammableSpeakerAlertParameters`](https://lua-api.factorio.com/latest/concepts/ProgrammableSpeakerAlertParameters.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SpeakerAlertParameters {
    pub show_alert: bool,
    pub show_on_map: bool,
    pub icon_signal_id: Option<SignalID>,
    pub alert_message: String,
}

/// [`VehicleAutomaticTargetingParameters`](https://lua-api.factorio.com/latest/concepts/VehicleAutomaticTargetingParameters.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VehicleAutomaticTargetingParameters {
    pub auto_target_without_gunner: bool,
    pub auto_target_with_gunner: bool,
}
