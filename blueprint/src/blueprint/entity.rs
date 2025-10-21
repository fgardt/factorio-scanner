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

    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<EntityExtraData>,
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
        ids.merge(self.extra_data.get_ids());

        ids
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EntityExtraData {
    //None {},
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
        filters: IndexedVec<ItemFilter>,
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
        rail_layer: Option<String>,

        control_behavior: Option<Box<RailSignalControlBehavior>>,
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

impl EntityExtraData {
    #[must_use]
    pub const fn recipe(&self) -> Option<&RecipeID> {
        match self {
            Self::AssemblingMachine { recipe, .. } | Self::RocketSilo { recipe, .. } => {
                recipe.as_ref()
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn recipe_quality(&self) -> Option<&QualityID> {
        match self {
            Self::AssemblingMachine { recipe_quality, .. }
            | Self::RocketSilo { recipe_quality, .. } => Some(recipe_quality),
            _ => None,
        }
    }

    #[must_use]
    pub const fn pickup_position(&self) -> Option<&Vector> {
        match self {
            Self::Inserter {
                pickup_position, ..
            } => pickup_position.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn drop_position(&self) -> Option<&Vector> {
        match self {
            Self::Inserter { drop_position, .. } => drop_position.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn input_priority(&self) -> Option<SplitterPriority> {
        match self {
            Self::LaneSplitter { input_priority, .. } | Self::Splitter { input_priority, .. } => {
                Some(*input_priority)
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn output_priority(&self) -> Option<SplitterPriority> {
        match self {
            Self::LaneSplitter {
                output_priority, ..
            }
            | Self::Splitter {
                output_priority, ..
            } => Some(*output_priority),
            _ => None,
        }
    }

    #[must_use]
    pub const fn splitter_filter(&self) -> Option<&ItemFilter> {
        match self {
            Self::LaneSplitter { filter, .. } | Self::Splitter { filter, .. } => filter.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn orientation(&self) -> Option<RealOrientation> {
        match self {
            Self::ArtilleryWagon { orientation, .. }
            | Self::CargoWagon { orientation, .. }
            | Self::InfinityCargoWagon { orientation, .. }
            | Self::Locomotive { orientation, .. } => Some(*orientation),
            _ => None,
        }
    }

    #[must_use]
    pub const fn color(&self) -> Option<&Color> {
        match self {
            Self::ArtilleryWagon { color, .. }
            | Self::CargoWagon { color, .. }
            | Self::FluidWagon { color, .. }
            | Self::Locomotive { color, .. }
            | Self::SpiderVehicle { color, .. }
            | Self::Lamp { color, .. } => color.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn combinator_data(&self) -> Option<&CombinatorData> {
        match self {
            Self::Combinator(data) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub const fn belt_connection_type(&self) -> Option<BeltConnectionType> {
        match self {
            Self::LinkedBelt { kind, .. }
            | Self::Loader { kind, .. }
            | Self::UndergroundBelt { kind, .. } => Some(*kind),
            _ => None,
        }
    }

    #[must_use]
    pub const fn rail_layer(&self) -> Option<&String> {
        match self {
            Self::RailSignal { rail_layer, .. } => rail_layer.as_ref(),
            _ => None,
        }
    }
}

impl crate::GetIDs for EntityExtraData {
    #[allow(clippy::too_many_lines)]
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();

        match self {
            //Self::None {} |
            Self::ElectricEnergyInterface { .. }
            | Self::HeatInterface { .. }
            | Self::LinkedBelt { .. }
            | Self::LinkedContainer { .. }
            | Self::ProxyContainer { .. }
            | Self::StorageTank { .. }
            | Self::UndergroundBelt { .. }
            | Self::Valve { .. } => {}
            Self::Accumulator { control_behavior } => ids.merge(control_behavior.get_ids()),
            Self::AgriculturalTower { control_behavior } => ids.merge(control_behavior.get_ids()),
            Self::ArtillertyTurret {
                control_behavior, ..
            } => {
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::ArtilleryWagon { grid_data, .. }
            | Self::FluidWagon { grid_data, .. }
            | Self::Locomotive { grid_data, .. } => ids.merge(grid_data.get_ids()),
            Self::AssemblingMachine {
                recipe,
                recipe_quality,
                control_behavior,
            } => {
                ids.merge(recipe.get_ids());
                ids.quality.insert(recipe_quality.clone());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::AsteroidCollector {
                control_behavior, ..
            } => {
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::Car {
                request_filters,
                grid_data,
                trunk_inventory,
                ammo_inventory,
                ..
            }
            | Self::SpiderVehicle {
                request_filters,
                grid_data,
                trunk_inventory,
                ammo_inventory,
                ..
            } => {
                ids.merge(request_filters.get_ids());
                ids.merge(grid_data.get_ids());
                ids.merge(trunk_inventory.get_ids());
                ids.merge(ammo_inventory.get_ids());
            }
            Self::CargoLandingPad {
                request_filters, ..
            } => ids.merge(request_filters.get_ids()),
            Self::CargoWagon {
                grid_data,
                inventory,
                ..
            } => {
                ids.merge(grid_data.get_ids());
                ids.merge(inventory.get_ids());
            }
            Self::Combinator(combinator_data) => ids.merge(combinator_data.get_ids()),
            Self::Container { filters, .. } | Self::Loader { filters, .. } => {
                ids.merge(filters.get_ids());
            }
            Self::DisplayPanel {
                icon,
                control_behavior,
                ..
            } => {
                ids.merge(icon.get_ids());
                ids.merge(control_behavior.get_ids());
            }
            Self::Furnace { control_behavior } => ids.merge(control_behavior.get_ids()),
            Self::InfinityCargoWagon {
                grid_data,
                inventory,
                infinity_settings,
                ..
            } => {
                ids.merge(grid_data.get_ids());
                ids.merge(inventory.get_ids());
                ids.merge(infinity_settings.get_ids());
            }
            Self::InfinityContainer {
                filters,
                request_filters,
                infinity_settings,
                ..
            } => {
                ids.merge(filters.get_ids());
                ids.merge(request_filters.get_ids());
                ids.merge(infinity_settings.get_ids());
            }
            Self::InfinityPipe { infinity_settings } => ids.merge(infinity_settings.get_ids()),
            Self::Inserter {
                filters,
                control_behavior,
                ..
            } => {
                ids.merge(filters.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::Lamp {
                control_behavior, ..
            } => {
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::LaneSplitter { filter, .. } => ids.merge(filter.get_ids()),
            Self::LogisticContainer {
                filters,
                request_filters,
                control_behavior,
                ..
            } => {
                ids.merge(filters.get_ids());
                ids.merge(request_filters.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::MiningDrill {
                filter,
                control_behavior,
            } => {
                ids.merge(filter.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::PowerSwitch {
                control_behavior, ..
            } => {
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::ProgrammableSpeaker {
                parameters,
                alert_parameters,
                control_behavior,
            } => {
                ids.merge(parameters.get_ids());
                ids.merge(alert_parameters.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::Pump {
                fluid_filter,
                control_behavior,
            } => {
                ids.merge(fluid_filter.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::RailSignal {
                control_behavior, ..
            } => ids.merge(control_behavior.get_ids()),
            Self::Reactor { control_behavior } => ids.merge(control_behavior.get_ids()),
            Self::Roboport {
                request_filters,
                control_behavior,
            } => {
                ids.merge(request_filters.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::RocketSilo {
                recipe,
                recipe_quality,
                ..
            } => {
                ids.merge(recipe.get_ids());
                ids.quality.insert(recipe_quality.clone());
            }
            Self::SpacePlatformHub {
                request_filters,
                control_behavior,
                ..
            } => {
                ids.merge(request_filters.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::Splitter {
                filter,
                control_behavior,
                ..
            } => {
                ids.merge(filter.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::TrainStop {
                control_behavior, ..
            } => {
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::TransportBelt { control_behavior } => ids.merge(control_behavior.get_ids()),
            Self::Turret {
                priority_list,
                control_behavior,
                ..
            } => {
                ids.merge(priority_list.get_ids());
                if let Some(cb) = control_behavior {
                    ids.merge(cb.get_ids());
                }
            }
            Self::Wall { control_behavior } => ids.merge(control_behavior.get_ids()),
        }

        ids
    }
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

impl CombinatorData {
    #[must_use]
    pub const fn control_behavior(&self) -> Option<&CombinatorControlBehavior> {
        match &self.control_behavior {
            Some(cb) => Some(cb),
            None => None,
        }
    }

    #[must_use]
    pub const fn arithmetic_parameters(&self) -> Option<&ArithmeticCombinatorParameters> {
        match self.control_behavior() {
            Some(CombinatorControlBehavior::Arithmetic {
                arithmetic_conditions,
            }) => Some(arithmetic_conditions),
            _ => None,
        }
    }

    #[must_use]
    pub const fn decider_parameters(&self) -> Option<&DeciderCombinatorParameters> {
        match self.control_behavior() {
            Some(CombinatorControlBehavior::Decider { decider_conditions }) => {
                Some(decider_conditions)
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn selector_parameters(&self) -> Option<&SelectorCombinatorParameters> {
        match self.control_behavior() {
            Some(CombinatorControlBehavior::Selector(selector_params)) => Some(selector_params),
            _ => None,
        }
    }
}

impl crate::GetIDs for CombinatorData {
    fn get_ids(&self) -> crate::UsedIDs {
        self.control_behavior.get_ids()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GridData {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub enable_logistics_while_moving: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub grid: Vec<BlueprintEquipment>,
}

impl crate::GetIDs for GridData {
    fn get_ids(&self) -> crate::UsedIDs {
        self.grid.get_ids()
    }
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

impl crate::GetIDs for InfinityPipeFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        let mut ids = crate::UsedIDs::default();
        ids.fluid.insert(self.name.clone());
        ids
    }
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

impl crate::GetIDs for MiningDrillFilter {
    fn get_ids(&self) -> crate::UsedIDs {
        self.filters.get_ids()
    }
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

impl crate::GetIDs for SpeakerParameters {
    fn get_ids(&self) -> crate::UsedIDs {
        self.volume_signal_id.get_ids()
    }
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

impl crate::GetIDs for SpeakerAlertParameters {
    fn get_ids(&self) -> crate::UsedIDs {
        self.icon_signal_id.get_ids()
    }
}

/// [`VehicleAutomaticTargetingParameters`](https://lua-api.factorio.com/latest/concepts/VehicleAutomaticTargetingParameters.html)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VehicleAutomaticTargetingParameters {
    pub auto_target_without_gunner: bool,
    pub auto_target_with_gunner: bool,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::bp_string_to_json;

    #[test]
    fn omni_crafter_entities() {
        let raw_omni = bp_string_to_json(include_str!("../../tests/omni_crafter_GWP.txt")).unwrap();
        let json_data = serde_json::from_str::<serde_json::Value>(&raw_omni).unwrap();

        let bps = json_data["blueprint_book"]["blueprints"]
            .as_array()
            .unwrap();

        for entry in bps {
            let entities = entry["blueprint"]["entities"].as_array().unwrap();

            for raw_entity in entities {
                let entity_json = serde_json::to_string_pretty(raw_entity).unwrap();
                let Err(e) = serde_json::from_str::<Entity>(&entity_json) else {
                    continue;
                };

                panic!("Failed to deserialize entity {e:?}: {entity_json}");
            }
        }
    }
}
