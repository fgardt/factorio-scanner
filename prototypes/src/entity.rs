use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::BasePrototype;
use mod_util::UsedMods;
use types::*;

mod abstractions;
use abstractions::*;

mod accumulator;
mod artillery_turret;
mod beacon;
mod boiler;
mod burner_generator;
mod combinators;
mod containers;
mod crafting_machines;
mod electric_energy_interface;
mod electric_pole;
mod flying_robots;
mod gate;
mod generator;
mod heat_interface;
mod heat_pipe;
mod inserter;
mod lab;
mod lamp;
mod landmine;
mod market;
mod mining_drill;
mod offshore_pump;
mod pipe;
mod power_switch;
mod programmable_speaker;
mod pump;
mod radar;
mod rail_signals;
mod rails;
mod reactor;
mod roboport;
mod simple_entities;
mod solar_panel;
mod storage_tank;
mod train_stop;
mod transport_belts;
mod turrets;
mod vehicles;
mod wall;

pub use accumulator::*;
pub use artillery_turret::*;
pub use beacon::*;
pub use boiler::*;
pub use burner_generator::*;
pub use combinators::*;
pub use containers::*;
pub use crafting_machines::*;
pub use electric_energy_interface::*;
pub use electric_pole::*;
pub use flying_robots::*;
pub use gate::*;
pub use generator::*;
pub use heat_interface::*;
pub use heat_pipe::*;
pub use inserter::*;
pub use lab::*;
pub use lamp::*;
pub use landmine::*;
pub use market::*;
pub use mining_drill::*;
pub use offshore_pump::*;
pub use pipe::*;
pub use power_switch::*;
pub use programmable_speaker::*;
pub use pump::*;
pub use radar::*;
pub use rail_signals::*;
pub use rails::*;
pub use reactor::*;
pub use roboport::*;
pub use simple_entities::*;
pub use solar_panel::*;
pub use storage_tank::*;
pub use train_stop::*;
pub use transport_belts::*;
pub use turrets::*;
pub use vehicles::*;
pub use wall::*;

#[derive(Debug, Clone, Default)]
pub struct RenderOpts {
    pub position: MapPosition,

    pub direction: Direction,
    pub orientation: Option<RealOrientation>,

    pub pickup_position: Option<Vector>,

    pub connections: Option<ConnectedDirections>,

    pub underground_in: Option<bool>,

    pub connected_gates: Vec<Direction>,
    pub draw_gate_patch: bool,

    pub arithmetic_operation: Option<ArithmeticOperation>,
    pub decider_operation: Option<Comparator>,

    pub runtime_tint: Option<Color>,

    pub entity_id: u64,
    pub circuit_connected: bool,
    pub logistic_connected: bool,

    pub fluid_recipe: (bool, bool),
}

// From impls for RenderOpts variants from types
impl From<&RenderOpts> for SimpleGraphicsRenderOpts {
    fn from(opts: &RenderOpts) -> Self {
        Self {
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for RotatedSpriteRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            orientation: value.clone().orientation.unwrap_or_default(),
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for SpriteNWayRenderOpts {
    fn from(opts: &RenderOpts) -> Self {
        Self {
            direction: opts.direction,
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for SpriteVariationsRenderOpts {
    fn from(opts: &RenderOpts) -> Self {
        Self {
            variation: 0,
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for AnimationRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for Animation4WayRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            direction: value.direction,
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for AnimationVariationsRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            variation: 0,
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for RotatedAnimationRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            orientation: value
                .orientation
                .unwrap_or_else(|| value.direction.to_orientation()),
            progress: 0.0,
            runtime_tint: value.runtime_tint,
            override_index: None,
        }
    }
}

impl From<&RenderOpts> for RotatedAnimation4WayRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            direction: value.direction,
            orientation: value.orientation.unwrap_or_default(),
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for BeaconGraphicsSetRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            runtime_tint: value.runtime_tint,
        }
    }
}

impl From<&RenderOpts> for TransportBeltAnimationSetRenderOpts {
    fn from(opts: &RenderOpts) -> Self {
        Self {
            direction: opts.direction,
            connections: opts.connections,

            runtime_tint: opts.runtime_tint,

            index_override: None,
        }
    }
}

impl From<&RenderOpts> for MiningDrillGraphicsRenderOpts {
    fn from(value: &RenderOpts) -> Self {
        Self {
            direction: value.direction,
            runtime_tint: value.runtime_tint,
        }
    }
}

pub type RenderOutput = Option<()>;

pub trait Renderable {
    fn render(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> RenderOutput;

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition>;
    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition>;
}

/// [`Prototypes/EntityPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityPrototype<T: Renderable>(BasePrototype<EntityData<T>>);

impl<T: Renderable> Deref for EntityPrototype<T> {
    type Target = BasePrototype<EntityData<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Renderable> Renderable for EntityPrototype<T> {
    fn render(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}

pub trait RenderableEntity: Renderable {
    fn collision_box(&self) -> BoundingBox;
    fn selection_box(&self) -> BoundingBox;
    fn drawing_box(&self) -> BoundingBox;

    fn pipe_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)>;
    fn heat_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)>;
}

impl<R, T> RenderableEntity for T
where
    R: Renderable,
    T: Renderable + Deref<Target = BasePrototype<EntityData<R>>>,
{
    fn collision_box(&self) -> BoundingBox {
        self.collision_box.clone().unwrap_or_default()
    }

    fn selection_box(&self) -> BoundingBox {
        self.selection_box.clone().unwrap_or_default()
    }

    fn drawing_box(&self) -> BoundingBox {
        self.drawing_box
            .clone()
            .unwrap_or_else(|| self.selection_box())
    }

    fn pipe_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)> {
        let raw_connections = self.fluid_box_connections(options);

        if raw_connections.is_empty() {
            return Vec::new();
        }

        let BoundingBox(tl, br) = self.collision_box();
        let tl_vec: Vector = tl.into();
        let br_vec: Vector = br.into();
        let (tl_x, tl_y) = options.direction.rotate_vector(tl_vec).as_tuple();
        let (br_x, br_y) = options.direction.rotate_vector(br_vec).as_tuple();

        let top_y = tl_y.min(br_y);
        let bottom_y = tl_y.max(br_y);
        let left_x = tl_x.min(br_x);
        let right_x = tl_x.max(br_x);

        raw_connections
            .iter()
            .filter_map(|conn| {
                let conn = conn.clone();
                let (x, y) = conn.as_tuple();

                let dir = if y <= top_y {
                    Direction::South
                } else if y >= bottom_y {
                    Direction::North
                } else if x <= left_x {
                    Direction::East
                } else if x >= right_x {
                    Direction::West
                } else {
                    println!(
                        "Invalid pipe connection [{}] @ {:?}: {conn:?}",
                        self.name, options.direction
                    );
                    return None;
                };

                Some((conn + &options.position, dir))
            })
            .collect()
    }

    fn heat_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)> {
        let raw_connections = self.heat_buffer_connections(options);

        if raw_connections.is_empty() {
            return Vec::new();
        }

        let BoundingBox(tl, br) = self.collision_box();
        let tl_vec: Vector = tl.into();
        let br_vec: Vector = br.into();
        let (tl_x, tl_y) = options.direction.rotate_vector(tl_vec).as_tuple();
        let (br_x, br_y) = options.direction.rotate_vector(br_vec).as_tuple();

        let top_y = tl_y.min(br_y);
        let bottom_y = tl_y.max(br_y);
        let left_x = tl_x.min(br_x);
        let right_x = tl_x.max(br_x);

        raw_connections
            .iter()
            .filter_map(|conn| {
                let conn = conn.clone();
                let (x, y) = conn.as_tuple();

                let dir = if y <= top_y {
                    Direction::South
                } else if y >= bottom_y {
                    Direction::North
                } else if x <= left_x {
                    Direction::East
                } else if x >= right_x {
                    Direction::West
                } else {
                    println!(
                        "Invalid heat connection [{}] @ {:?}: {conn:?}",
                        self.name, options.direction
                    );
                    return None;
                };

                Some((conn + &options.position, dir))
            })
            .collect()
    }
}

/// [`Prototypes/EntityPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityData<T: Renderable> {
    #[serde(flatten)]
    pub icon: Option<Icon>,

    pub collision_box: Option<BoundingBox>,
    pub collision_mask: Option<CollisionMask>,

    pub map_generator_bounding_box: Option<BoundingBox>,
    pub selection_box: Option<BoundingBox>,
    pub drawing_box: Option<BoundingBox>,
    pub sticker_box: Option<BoundingBox>,
    pub hit_visualization_box: Option<BoundingBox>,

    // TODO: get a proper default and serializing skip (?)
    //#[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Option<EntityPrototypeFlags>,
    pub subgroup: Option<ItemSubGroupID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_copy_paste: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub selectable_in_game: bool,

    #[serde(
        default = "helper::u8_50",
        skip_serializing_if = "helper::is_50_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub selection_priority: u8,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub build_grid_size: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub remove_decoratives: DecorativeRemoveMode,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub emissions_per_second: f64,

    pub shooting_cursor_size: Option<f32>,

    pub radius_visualisation_specification: Option<RadiusVisualisationSpecification>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub build_base_evolution_requirement: f64,

    pub alert_icon_shift: Option<Vector>,

    pub alert_icon_scale: Option<f64>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fast_replaceable_group: String,

    pub next_upgrade: Option<EntityID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub protected_from_tile_building: bool,

    pub placeable_by: Option<PlaceableBy>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_pastable_entities: FactorioArray<EntityID>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub tile_width: Option<u32>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub tile_height: Option<u32>,

    pub map_color: Option<Color>,
    pub friendly_map_color: Option<Color>,
    pub enemy_map_color: Option<Color>,

    pub water_reflection: Option<WaterReflectionDefinition>,
    // not implemented
    // pub trigger_target_mask: Option<TriggerTargetMask>,
    // pub minable: Option<MinableProperties>,
    // pub created_smoke: Option<CreateTrivialSmokeEffectItem>,
    // pub working_sound: Option<WorkingSound>,
    // pub created_effect: Option<Trigger>,
    // pub build_sound: Option<Sound>,
    // pub mined_sound: Option<Sound>,
    // pub mining_sound: Option<Sound>,
    // pub rotated_sound: Option<Sound>,
    // pub vehicle_impact_sound: Option<Sound>,
    // pub open_sound: Option<Sound>,
    // pub close_sound: Option<Sound>,
    // pub remains_when_mined: Option<RemainsWhenMined>,
    // pub autoplace: Option<AutoplaceSpecification>,
    #[serde(flatten)]
    child: T,
}

impl<T: Renderable> Deref for EntityData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for EntityData<T> {
    fn render(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum DecorativeRemoveMode {
    #[default]
    Automatic,
    True,
    False,
}

impl serde::ser::Serialize for DecorativeRemoveMode {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automatic => serializer.serialize_str("automatic"),
            Self::True => serializer.serialize_str("true"),
            Self::False => serializer.serialize_str("false"),
        }
    }
}

struct DecorativeRemoveModeVisitor;

impl<'de> serde::de::Visitor<'de> for DecorativeRemoveModeVisitor {
    type Value = DecorativeRemoveMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("one of 'automatic', 'true' or 'false'")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(if value {
            DecorativeRemoveMode::True
        } else {
            DecorativeRemoveMode::False
        })
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "automatic" => Ok(DecorativeRemoveMode::Automatic),
            "true" => Ok(DecorativeRemoveMode::True),
            "false" => Ok(DecorativeRemoveMode::False),
            _ => Err(serde::de::Error::unknown_variant(
                value,
                &["automatic", "true", "false"],
            )),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for DecorativeRemoveMode {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(DecorativeRemoveModeVisitor)
    }
}

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
pub type EntityWithHealthPrototype<T> = EntityPrototype<EntityWithHealthData<T>>;

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithHealthData<T: Renderable> {
    #[serde(default = "helper::f64_10", skip_serializing_if = "helper::is_10_f64")]
    pub max_health: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub healing_per_tick: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub repair_speed_modifier: f64,

    //#[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resistances: Option<Resistances>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub alert_when_damaged: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub hide_resistances: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub create_ghost_on_death: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub random_corpse_variation: bool,

    pub integration_patch_render_layer: Option<RenderLayer>,
    pub integration_patch: Option<Sprite4Way>,
    // not implemented
    // pub dying_explosion: Option<ExplosionDefinition>,
    // pub dying_trigger_effect: Option<TriggerEffect>,
    // pub damaged_trigger_effect: Option<TriggerEffect>,
    // pub loot: FactorioArray<LootItem>,
    // pub attack_reaction: AttackReactionItem or FactorioArray<AttackReactionItem>,
    // pub repair_sound: Option<Sound>,
    // pub corpse: Option<Corpse>,
    #[serde(flatten)]
    child: T,
}

impl<T: Renderable> Deref for EntityWithHealthData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for EntityWithHealthData<T> {
    fn render(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
pub type EntityWithOwnerPrototype<T> = EntityWithHealthPrototype<EntityWithOwnerData<T>>;

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithOwnerData<T: Renderable> {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_military_target: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_run_time_change_of_is_military_target: bool,

    #[serde(flatten)]
    child: T,
}

impl<T: Renderable> Deref for EntityWithOwnerData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for EntityWithOwnerData<T> {
    fn render(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Accumulator,
    ArtilleryTurret,
    Beacon,
    Boiler,
    BurnerGenerator,
    ArithmeticCombinator,
    DeciderCombinator,
    ConstantCombinator,
    ProgrammableSpeaker,
    Container,
    LogisticContainer,
    InfinityContainer,
    LinkedContainer,
    AssemblingMachine,
    RocketSilo,
    Furnace,
    ElectricEnergyInterface,
    ElectricPole,
    PowerSwitch,
    CombatRobot,
    ConstructionRobot,
    LogisticRobot,
    Roboport,
    Gate,
    Wall,
    Generator,
    Reactor,
    HeatInterface,
    HeatPipe,
    Inserter,
    Lab,
    Lamp,
    LandMine,
    Market,
    MiningDrill,
    OffshorePump,
    Pipe,
    InfinityPipe,
    PipeToGround,
    Pump,
    SimpleEntityWithOwner,
    SimpleEntityWithForce,
    SolarPanel,
    StorageTank,
    LinkedBelt,
    Loader1x1,
    Loader,
    Splitter,
    TransportBelt,
    UndergroundBelt,
    Radar,
    Turret,
    AmmoTurret,
    ElectricTurret,
    FluidTurret,
    Car,
    CurvedRail,
    StraightRail,
    RailSignal,
    RailChainSignal,
    TrainStop,
    Locomotive,
    CargoWagon,
    FluidWagon,
    ArtilleryWagon,
}

#[allow(clippy::match_like_matches_macro)]
impl Type {
    #[must_use]
    pub const fn connectable(&self) -> bool {
        match self {
            Self::HeatPipe | Self::HeatInterface => true,
            Self::Pipe | Self::InfinityPipe => true,
            Self::TransportBelt => true,
            Self::Wall | Self::Gate => true,
            _ => false,
        }
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub const fn can_connect_to(&self, other: &Self) -> bool {
        match self {
            Self::Gate => matches!(other, Self::Wall),
            Self::Wall => match other {
                Self::Wall => true,
                Self::Gate => true, // when direction fits
                _ => false,
            },
            Self::TransportBelt => match other {
                Self::Loader | Self::Loader1x1 => true,
                Self::UndergroundBelt => true,
                Self::TransportBelt => true,
                Self::LinkedBelt => true,
                Self::Splitter => true,
                _ => false,
            },
            Self::Pipe | Self::InfinityPipe => match other {
                Self::Pipe | Self::InfinityPipe | Self::PipeToGround => true,
                Self::Pump | Self::OffshorePump | Self::StorageTank => true,
                Self::AssemblingMachine | Self::Furnace => true,
                Self::Boiler | Self::Generator => true,
                Self::MiningDrill => true,
                Self::FluidTurret => true,
                _ => false,
            },
            Self::HeatPipe | Self::HeatInterface => match other {
                Self::HeatPipe | Self::HeatInterface => true,
                Self::Reactor => true,
                Self::Boiler
                | Self::Inserter
                | Self::AssemblingMachine
                | Self::Furnace
                | Self::Lab
                | Self::MiningDrill
                | Self::Pump
                | Self::Radar => true, // when energy_source.type == "heat"
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityPrototypeMap<T: Renderable>(HashMap<String, T>);

impl<T> Default for EntityPrototypeMap<T>
where
    T: Renderable,
{
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Renderable> Deref for EntityPrototypeMap<T> {
    type Target = HashMap<String, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AllTypes {
    pub accumulator: EntityPrototypeMap<AccumulatorPrototype>,
    pub artillery_turret: EntityPrototypeMap<ArtilleryTurretPrototype>,
    pub beacon: EntityPrototypeMap<BeaconPrototype>,
    pub boiler: EntityPrototypeMap<BoilerPrototype>,
    pub burner_generator: EntityPrototypeMap<BurnerGeneratorPrototype>,

    pub arithmetic_combinator: EntityPrototypeMap<ArithmeticCombinatorPrototype>,
    pub decider_combinator: EntityPrototypeMap<DeciderCombinatorPrototype>,
    pub constant_combinator: EntityPrototypeMap<ConstantCombinatorPrototype>,
    pub programmable_speaker: EntityPrototypeMap<ProgrammableSpeakerPrototype>,

    pub container: EntityPrototypeMap<ContainerPrototype>,
    pub logistic_container: EntityPrototypeMap<LogisticContainerPrototype>,
    pub infinity_container: EntityPrototypeMap<InfinityContainerPrototype>,
    pub linked_container: EntityPrototypeMap<LinkedContainerPrototype>,

    pub assembling_machine: EntityPrototypeMap<AssemblingMachinePrototype>,
    pub rocket_silo: EntityPrototypeMap<RocketSiloPrototype>,
    pub furnace: EntityPrototypeMap<FurnacePrototype>,

    pub electric_energy_interface: EntityPrototypeMap<ElectricEnergyInterfacePrototype>,
    pub electric_pole: EntityPrototypeMap<ElectricPolePrototype>,
    pub power_switch: EntityPrototypeMap<PowerSwitchPrototype>,

    pub combat_robot: EntityPrototypeMap<CombatRobotPrototype>,
    pub construction_robot: EntityPrototypeMap<ConstructionRobotPrototype>,
    pub logistic_robot: EntityPrototypeMap<LogisticRobotPrototype>,
    pub roboport: EntityPrototypeMap<RoboportPrototype>,

    pub gate: EntityPrototypeMap<GatePrototype>,
    pub wall: EntityPrototypeMap<WallPrototype>,

    pub generator: EntityPrototypeMap<GeneratorPrototype>,

    pub reactor: EntityPrototypeMap<ReactorPrototype>,
    pub heat_interface: EntityPrototypeMap<HeatInterfacePrototype>,
    pub heat_pipe: EntityPrototypeMap<HeatPipePrototype>,

    pub inserter: EntityPrototypeMap<InserterPrototype>,

    pub lab: EntityPrototypeMap<LabPrototype>,

    pub lamp: EntityPrototypeMap<LampPrototype>,

    pub land_mine: EntityPrototypeMap<LandMinePrototype>,

    pub market: EntityPrototypeMap<MarketPrototype>,

    pub mining_drill: EntityPrototypeMap<MiningDrillPrototype>,
    pub offshore_pump: EntityPrototypeMap<OffshorePumpPrototype>,

    pub pipe: EntityPrototypeMap<PipePrototype>,
    pub infinity_pipe: EntityPrototypeMap<InfinityPipePrototype>,
    pub pipe_to_ground: EntityPrototypeMap<PipeToGroundPrototype>,
    pub pump: EntityPrototypeMap<PumpPrototype>,

    pub simple_entity: EntityPrototypeMap<SimpleEntityPrototype>,
    pub simple_entity_with_owner: EntityPrototypeMap<SimpleEntityWithOwnerPrototype>,
    pub simple_entity_with_force: EntityPrototypeMap<SimpleEntityWithForcePrototype>,

    pub solar_panel: EntityPrototypeMap<SolarPanelPrototype>,

    pub storage_tank: EntityPrototypeMap<StorageTankPrototype>,

    pub linked_belt: EntityPrototypeMap<LinkedBeltPrototype>,
    pub loader_1x1: EntityPrototypeMap<Loader1x1Prototype>,
    pub loader: EntityPrototypeMap<Loader1x2Prototype>,
    pub splitter: EntityPrototypeMap<SplitterPrototype>,
    pub transport_belt: EntityPrototypeMap<TransportBeltPrototype>,
    pub underground_belt: EntityPrototypeMap<UndergroundBeltPrototype>,

    pub radar: EntityPrototypeMap<RadarPrototype>,
    pub turret: EntityPrototypeMap<TurretPrototype>,
    pub ammo_turret: EntityPrototypeMap<AmmoTurretPrototype>,
    pub electric_turret: EntityPrototypeMap<ElectricTurretPrototype>,
    pub fluid_turret: EntityPrototypeMap<FluidTurretPrototype>,

    pub car: EntityPrototypeMap<CarPrototype>,

    pub curved_rail: EntityPrototypeMap<CurvedRailPrototype>,
    pub straight_rail: EntityPrototypeMap<StraightRailPrototype>,
    pub rail_signal: EntityPrototypeMap<RailSignalPrototype>,
    pub rail_chain_signal: EntityPrototypeMap<RailChainSignalPrototype>,
    pub train_stop: EntityPrototypeMap<TrainStopPrototype>,
    pub locomotive: EntityPrototypeMap<LocomotivePrototype>,
    pub cargo_wagon: EntityPrototypeMap<CargoWagonPrototype>,
    pub fluid_wagon: EntityPrototypeMap<FluidWagonPrototype>,
    pub artillery_wagon: EntityPrototypeMap<ArtilleryWagonPrototype>,
    // not implemented
    // pub character: EntityPrototypeMap<CharacterPrototype>,
    // pub unit_spawner: EntityPrototypeMap<EnemySpawnerPrototype>,
    // pub player_port: EntityPrototypeMap<PlayerPortPrototype>,
    // pub unit: EntityPrototypeMap<UnitPrototype>,
    // pub spider_vehicle: EntityPrototypeMap<SpiderVehiclePrototype>,
}
