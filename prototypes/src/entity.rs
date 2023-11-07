use std::{collections::HashMap, ops::Deref, path::Path};

use image::DynamicImage;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::BasePrototype;
use types::*;

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

#[derive(Debug, Clone)]
pub struct RenderOpts<'a> {
    pub factorio_dir: &'a Path,
    pub used_mods: mod_util::UsedMods,

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
}

impl<'a> Default for RenderOpts<'a> {
    fn default() -> Self {
        Self {
            factorio_dir: Path::new(""),
            used_mods: HashMap::default(),
            direction: Direction::default(),
            orientation: Option::default(),
            pickup_position: Option::default(),
            connections: Option::default(),
            underground_in: Option::default(),
            connected_gates: Vec::default(),
            draw_gate_patch: Default::default(),
            arithmetic_operation: Option::default(),
            decider_operation: Option::default(),
            runtime_tint: Option::default(),
        }
    }
}

// From impls for RenderOpts variants from types
impl<'a> From<&'a RenderOpts<'a>> for SimpleGraphicsRenderOpts {
    fn from(opts: &'a RenderOpts<'a>) -> Self {
        Self {
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for RotatedSpriteRenderOpts {
    fn from(value: &'a RenderOpts) -> Self {
        Self {
            orientation: value.clone().orientation.unwrap_or_default(),
            runtime_tint: value.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for SpriteNWayRenderOpts {
    fn from(opts: &'a RenderOpts<'a>) -> Self {
        Self {
            direction: opts.direction,
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for SpriteVariationsRenderOpts {
    fn from(opts: &'a RenderOpts) -> Self {
        Self {
            variation: 0,
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for AnimationRenderOpts {
    fn from(value: &'a RenderOpts<'a>) -> Self {
        Self {
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for Animation4WayRenderOpts {
    fn from(value: &'a RenderOpts) -> Self {
        Self {
            direction: value.direction,
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for AnimationVariationsRenderOpts {
    fn from(value: &'a RenderOpts) -> Self {
        Self {
            variation: 0,
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for RotatedAnimationRenderOpts {
    fn from(value: &'a RenderOpts<'a>) -> Self {
        Self {
            orientation: value.orientation.unwrap_or_default(),
            progress: 0.0,
            runtime_tint: value.runtime_tint,
            override_index: None,
        }
    }
}

impl<'a> From<&RenderOpts<'a>> for RotatedAnimation4WayRenderOpts {
    fn from(value: &RenderOpts<'a>) -> Self {
        Self {
            direction: value.direction,
            orientation: value.orientation.unwrap_or_default(),
            progress: 0.0,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for BeaconGraphicsSetRenderOpts {
    fn from(value: &'a RenderOpts) -> Self {
        Self {
            runtime_tint: value.runtime_tint,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for TransportBeltAnimationSetRenderOpts {
    fn from(opts: &'a RenderOpts) -> Self {
        Self {
            direction: opts.direction,
            connections: opts.connections,

            runtime_tint: opts.runtime_tint,

            index_override: None,
        }
    }
}

impl<'a> From<&'a RenderOpts<'a>> for MiningDrillGraphicsRenderOpts {
    fn from(value: &'a RenderOpts<'a>) -> Self {
        Self {
            direction: value.direction,
            runtime_tint: value.runtime_tint,
        }
    }
}

pub trait Renderable {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityPrototypeMap<T: Renderable>(HashMap<String, T>);

impl<T: Renderable> Deref for EntityPrototypeMap<T> {
    type Target = HashMap<String, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Prototypes/EntityPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityPrototype<T: Renderable>(BasePrototype<EntityData<T>>);

impl<T: Renderable> Renderable for EntityPrototype<T> {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)> {
        self.0.child.render(options, image_cache)
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

    // skip serializing if default
    #[serde(default)]
    pub remove_decoratives: DecorativeRemoveMode,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub emissions_per_second: f64,

    pub shooting_cursor_size: Option<f64>,

    pub radius_visualisation_specification: Option<RadiusVisualisationSpecification>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
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
    pub child: T,
}

impl<T: Renderable> Renderable for EntityData<T> {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)> {
        self.child.render(options, image_cache)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DecorativeRemoveMode {
    #[default]
    Automatic,
    True,
    False,
}

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithHealthPrototype<T: Renderable>(EntityData<EntityWithHealthData<T>>);

impl<T: Renderable> Renderable for EntityWithHealthPrototype<T> {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithHealthData<T: Renderable> {
    #[serde(default = "helper::f64_10", skip_serializing_if = "helper::is_10_f64")]
    pub max_health: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
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
    // pub attack_reaction: FactorioArray<AttackReactionItem>,
    // pub repair_sound: Option<Sound>,
    // pub corpse: Option<Corpse>,
    #[serde(flatten)]
    pub child: T,
}

impl<T: Renderable> Renderable for EntityWithHealthData<T> {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)> {
        self.child.render(options, image_cache)
    }
}

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithOwnerPrototype<T: Renderable>(
    EntityWithHealthPrototype<EntityWithOwnerData<T>>,
);

impl<T: Renderable> Renderable for EntityWithOwnerPrototype<T> {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)> {
        self.0.render(options, image_cache)
    }
}

/// [`Prototypes/EntityWithHealthPrototype`](https://lua-api.factorio.com/latest/prototypes/EntityWithHealthPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityWithOwnerData<T: Renderable> {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_military_target: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_run_time_change_of_is_military_target: bool,

    #[serde(flatten)]
    pub child: T,
}

impl<T: Renderable> Renderable for EntityWithOwnerData<T> {
    fn render(
        &self,
        options: &RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<(DynamicImage, f64, Vector)> {
        self.child.render(options, image_cache)
    }
}
