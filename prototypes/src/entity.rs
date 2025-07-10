use std::{collections::HashMap, num::NonZeroU32, ops::Deref};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use tracing::warn;

use crate::helper_macro::namespace_struct;

use super::BasePrototype;
use mod_util::UsedMods;
use types::*;

mod abstractions;
use abstractions::*;

mod accumulator;
mod artillery_turret;
mod asteroid_collector;
mod beacon;
mod boiler;
mod burner_generator;
mod combinators;
mod containers;
mod crafting_machines;
mod displaypanel;
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
mod lightning_attractor;
mod market;
mod mining_drill;
mod offshore_pump;
mod pipe;
mod power_switch;
mod programmable_speaker;
mod proxy;
mod pump;
mod radar;
mod rail_signals;
mod rails;
mod reactor;
mod roboport;
mod simple_entities;
mod solar_panel;
mod storage_tank;
mod thruster;
mod train_stop;
mod transport_belts;
mod turrets;
mod valve;
mod vehicles;
mod wall;

pub use accumulator::*;
pub use artillery_turret::*;
pub use asteroid_collector::*;
pub use beacon::*;
pub use boiler::*;
pub use burner_generator::*;
pub use combinators::*;
pub use containers::*;
pub use crafting_machines::*;
pub use displaypanel::*;
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
pub use lightning_attractor::*;
pub use market::*;
pub use mining_drill::*;
pub use offshore_pump::*;
pub use pipe::*;
pub use power_switch::*;
pub use programmable_speaker::*;
pub use proxy::*;
pub use pump::*;
pub use radar::*;
pub use rail_signals::*;
pub use rails::*;
pub use reactor::*;
pub use roboport::*;
pub use simple_entities::*;
pub use solar_panel::*;
pub use storage_tank::*;
pub use thruster::*;
pub use train_stop::*;
pub use transport_belts::*;
pub use turrets::*;
pub use valve::*;
pub use vehicles::*;
pub use wall::*;

#[derive(Debug, Clone, Default)]
pub struct RenderOpts {
    pub position: MapPosition,

    pub direction: Direction,
    pub orientation: Option<RealOrientation>,
    pub mirrored: bool,
    pub elevated: bool,

    pub variation: Option<NonZeroU32>,

    pub pickup_position: Option<Vector>,

    pub connections: Option<ConnectedDirections>,

    pub underground_in: Option<bool>,

    pub connected_gates: Vec<Direction>,
    pub draw_gate_patch: bool,

    pub arithmetic_operation: Option<ArithmeticOperation>,
    pub decider_operation: Option<Comparator>,
    pub selector_operation: Option<SelectorOperation>,

    pub runtime_tint: Option<Color>,

    pub entity_id: u64,
    pub circuit_connected: bool,
    pub logistic_connected: bool,

    pub fluid_recipe: (bool, bool),
}

// From impls for RenderOpts variants from types
impl From<&RenderOpts> for TintableRenderOpts {
    fn from(opts: &RenderOpts) -> Self {
        Self {
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for RotatedRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.orientation.unwrap_or_default(), opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for DirectionalRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.direction, opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for VariationRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        #[expect(unsafe_code)]
        Self::new(
            opts.variation
                .unwrap_or(unsafe { NonZeroU32::new_unchecked(1) }),
            opts.into(),
        )
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for AnimationRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(0.0, opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for LocationalRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.position, opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for ConnectedRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.connections, opts.into())
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

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }

    fn recipe_visible(&self) -> bool {
        false
    }

    fn render_debug(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        // empty default impl
    }
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

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }

    fn render_debug(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        render_layers.draw_box(&self.collision_box(), &options.position, [255, 0, 0, 255]);
        render_layers.draw_box(&self.selection_box(), &options.position, [255, 255, 0, 255]);

        render_layers.draw_dot(&options.position, [255, 0, 0, 255]);
        render_layers.draw_direction(&options.position, options.direction, [255, 0, 0, 255]);

        self.child.render_debug(options, used_mods, render_layers);
    }
}

pub trait RenderableEntity: Renderable {
    fn collision_box(&self) -> BoundingBox;
    fn selection_box(&self) -> BoundingBox;
    fn drawing_box(&self) -> BoundingBox;

    fn pipe_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)>;
    fn heat_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)>;

    fn show_recipe(&self) -> bool;
}

impl<T: Renderable> RenderableEntity for EntityPrototype<T> {
    fn collision_box(&self) -> BoundingBox {
        self.collision_box.clone().unwrap_or_default()
    }

    fn selection_box(&self) -> BoundingBox {
        self.selection_box.clone().unwrap_or_default()
    }

    fn drawing_box(&self) -> BoundingBox {
        self.collision_box()

        // TODO: figure out how this works in 2.0
        // self.drawing_box
        //     .clone()
        //     .unwrap_or_else(|| self.selection_box())
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
                    warn!(
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
                    warn!(
                        "Invalid heat connection [{}] @ {:?}: {conn:?}",
                        self.name, options.direction
                    );
                    return None;
                };

                Some((conn + &options.position, dir))
            })
            .collect()
    }

    fn show_recipe(&self) -> bool {
        self.recipe_visible()
    }
}

impl<T, R> RenderableEntity for T
where
    T: Deref<Target = EntityWithOwnerPrototype<R>> + Renderable,
    R: Renderable,
{
    fn collision_box(&self) -> BoundingBox {
        self.deref().collision_box()
    }

    fn selection_box(&self) -> BoundingBox {
        self.deref().selection_box()
    }

    fn drawing_box(&self) -> BoundingBox {
        self.deref().drawing_box()
    }

    fn pipe_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)> {
        self.deref().pipe_connections(options)
    }

    fn heat_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)> {
        self.deref().heat_connections(options)
    }

    fn show_recipe(&self) -> bool {
        self.deref().show_recipe()
    }
}

/// [`Prototypes/EntityPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityData<T: Renderable> {
    #[serde(flatten)]
    pub icon: Option<Icon>,

    pub collision_box: Option<BoundingBox>,
    pub collision_mask: Option<CollisionMaskConnector>,

    pub map_generator_bounding_box: Option<BoundingBox>,
    pub selection_box: Option<BoundingBox>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub drawing_box_vertical_extension: f64,
    pub sticker_box: Option<BoundingBox>,
    pub hit_visualization_box: Option<BoundingBox>,

    // TODO: get a proper default and serializing skip (?)
    //#[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Option<EntityPrototypeFlags>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surface_conditions: FactorioArray<SurfaceCondition>,

    pub deconstruction_alternative: Option<EntityID>,

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

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub emissions_per_second: HashMap<AirbornePollutantID, f64>,

    pub shooting_cursor_size: Option<f64>,

    pub impact_category: Option<String>,

    pub placeable_position_visualization: Option<Sprite>,

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

    #[serde(
        default = "default_heating_energy",
        skip_serializing_if = "is_default_heating_energy"
    )]
    pub heating_energy: Energy,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub allow_copy_paste: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub selectable_in_game: bool,

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
    // pub tile_buildability_rules: FactorioArray<TileBuildabilityRule>,
    // pub minable: Option<MinableProperties>,
    // pub created_smoke: Option<CreateTrivialSmokeEffectItem>,
    // pub working_sound: Option<WorkingSound>,
    // pub created_effect: Option<Trigger>,
    // pub build_sound: Option<Sound>,
    // pub mined_sound: Option<Sound>,
    // pub mining_sound: Option<Sound>,
    // pub rotated_sound: Option<Sound>,
    // pub open_sound: Option<Sound>,
    // pub close_sound: Option<Sound>,
    // pub stateless_visualisation: Option<StatelessVisualisations>,
    // pub remains_when_mined: Option<RemainsWhenMined>,
    // pub diagonal_tile_grid_size: Option<TilePosition>,
    // pub autoplace: Option<AutoplaceSpecification>,
    // pub ambient_sounds_group: Option<EntityID>,
    // pub ambient_sounds: Option<WorldAmbientSoundDefinitions>,
    // pub icon_draw_specification: Option<IconDrawSpecification>,
    // pub icons_positioning: FactorioArray<IconSequencePositioning>,
    #[serde(flatten)]
    child: T,
}

fn default_heating_energy() -> Energy {
    Energy::new("0W")
}

fn is_default_heating_energy(energy: &Energy) -> bool {
    energy == &default_heating_energy()
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

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }

    fn render_debug(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        self.child.render_debug(options, used_mods, render_layers);
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecorativeRemoveMode {
    #[default]
    Automatic,
    True,
    False,
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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub random_corpse_variation: bool,

    pub integration_patch_render_layer: Option<RenderLayer>,
    pub integration_patch: Option<Sprite4Way>,

    #[serde(
        default = "helper::f32_005",
        skip_serializing_if = "helper::is_005_f32"
    )]
    pub overkill_fraction: f32,
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
        let ret = self
            .child
            .render(options, used_mods, render_layers, image_cache);

        if let Some(patch) = self.integration_patch.as_ref() {
            if let Some(res) = patch.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            ) {
                render_layers.add(res, &options.position, crate::RenderLayer::GroundPatch);
                return Some(());
            }
        }

        ret
    }

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<MapPosition> {
        self.child.heat_buffer_connections(options)
    }

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }

    fn render_debug(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        self.child.render_debug(options, used_mods, render_layers);
    }
}

/// [`Prototypes/EntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithOwnerPrototype.html)
pub type EntityWithOwnerPrototype<T> = EntityWithHealthPrototype<EntityWithOwnerData<T>>;

/// [`Prototypes/EntityWithOwnerPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithOwnerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithOwnerData<T: Renderable> {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub is_military_target: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_run_time_change_of_is_military_target: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub quality_indicator_shift: Vector,

    pub quality_indicator_scale: Option<f64>,

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

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }

    fn render_debug(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        self.child.render_debug(options, used_mods, render_layers);
    }
}

#[allow(clippy::match_like_matches_macro)]
impl Type {
    #[must_use]
    pub const fn connectable(&self) -> bool {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::HeatPipe | Self::HeatInterface => true,
            Self::Pipe | Self::InfinityPipe => true,
            // Self::TransportBelt => true,
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
            // Self::TransportBelt => match other {
            //     Self::Loader | Self::Loader1x1 => true,
            //     Self::UndergroundBelt => true,
            //     Self::TransportBelt => true,
            //     Self::LinkedBelt => true,
            //     Self::Splitter => true,
            //     _ => false,
            // },
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
pub struct EntityPrototypeMap<T: Renderable>(HashMap<EntityID, T>);

impl<T> Default for EntityPrototypeMap<T>
where
    T: Renderable,
{
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Renderable> Deref for EntityPrototypeMap<T> {
    type Target = HashMap<EntityID, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// workaround for prototype type string not matching the actual type name
// type LoaderPrototype = Loader1x2Prototype;

namespace_struct! {
    AllTypes,
    EntityID,
    &dyn RenderableEntity,
    "accumulator",
    "artillery-turret",
    "asteroid-collector",
    "beacon",
    "boiler",
    "burner-generator",
    "arithmetic-combinator",
    "decider-combinator",
    "selector-combinator",
    "constant-combinator",
    "programmable-speaker",
    "display-panel",
    "container",
    "logistic-container",
    "infinity-container",
    "linked-container",
    "assembling-machine",
    "rocket-silo",
    "furnace",
    "electric-energy-interface",
    "electric-pole",
    "power-switch",
    "combat-robot",
    "construction-robot",
    "logistic-robot",
    "roboport",
    "gate",
    "wall",
    "generator",
    "reactor",
    "heat-interface",
    "heat-pipe",
    "inserter",
    "lab",
    "lamp",
    "land-mine",
    "lightning-attractor",
    "market",
    "mining-drill",
    "offshore-pump",
    "pipe",
    "infinity-pipe",
    "pipe-to-ground",
    "pump",
    "valve",
    "simple-entity",
    "simple-entity-with-owner",
    "simple-entity-with-force",
    "solar-panel",
    "storage-tank",
    "thruster",
    "linked-belt",
    "loader-1x1",
    "loader",
    "splitter",
    "lane-splitter",
    "transport-belt",
    "underground-belt",
    "radar",
    "turret",
    "ammo-turret",
    "electric-turret",
    "fluid-turret",
    "car",
    "proxy-container",
    "curved-rail-a",
    "curved-rail-b",
    "half-diagonal-rail",
    "straight-rail",
    "rail-ramp",
    "elevated-curved-rail-a",
    "elevated-curved-rail-b",
    "elevated-half-diagonal-rail",
    "elevated-straight-rail",
    "rail-signal",
    "rail-chain-signal",
    "train-stop",
    "locomotive",
    "cargo-wagon",
    "fluid-wagon",
    "artillery-wagon",
    "infinity-cargo-wagon",

    "legacy-curved-rail",
    "legacy-straight-rail"

    // not implemented
    // character,
    // unit-spawner,
    // player-port,
    // unit,
    // spider-vehicle,
}
