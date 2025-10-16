use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/FlyingRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/FlyingRobotPrototype.html)
pub type FlyingRobotPrototype<T> = EntityWithOwnerPrototype<FlyingRobotData<T>>;

/// [`Prototypes/FlyingRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/FlyingRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FlyingRobotData<T: super::Renderable> {
    pub speed: f64,

    #[serde(
        default = "helper::f64_max",
        skip_serializing_if = "helper::is_max_f64"
    )]
    pub max_speed: f64,

    // TODO: proper defaults for all energy fields ("0")
    pub max_energy: Option<Energy>,
    pub energy_per_move: Option<Energy>,
    pub energy_per_tick: Option<Energy>,

    #[serde(default = "helper::f64_02", skip_serializing_if = "helper::is_02_f64")]
    pub min_to_charge: f64,

    #[serde(
        default = "helper::f64_095",
        skip_serializing_if = "helper::is_095_f64"
    )]
    pub max_to_charge: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub speed_multiplier_when_out_of_energy: f64,

    #[serde(flatten)]
    child: T,
}

impl<T: super::Renderable> Deref for FlyingRobotData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for FlyingRobotData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }
}

/// [`Prototypes/CombatRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/CombatRobotPrototype.html)
pub type CombatRobotPrototype = FlyingRobotPrototype<CombatRobotData>;

/// [`Prototypes/CombatRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/CombatRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CombatRobotData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub time_to_live: u32,

    pub attack_parameters: AttackParameters,

    pub idle: RotatedAnimation,
    pub shadow_idle: RotatedAnimation,
    pub in_motion: RotatedAnimation,
    pub shadow_in_motion: RotatedAnimation,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub range_from_player: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub friction: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub follows_player: bool,

    pub light: Option<LightDefinition>,
    // not implemented
    // pub destroy_action: Option<Trigger>,
}

impl super::Renderable for CombatRobotData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }
}

/// [`Prototypes/RobotWithLogisticInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/RobotWithLogisticInterfacePrototype.html)
pub type RobotWithLogisticInterfacePrototype<T> =
    FlyingRobotPrototype<RobotWithLogisticInterfaceData<T>>;

/// [`Prototypes/RobotWithLogisticInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/RobotWithLogisticInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RobotWithLogisticInterfaceData<T: super::Renderable> {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_payload_size: ItemCountType,
    pub max_payload_size_after_bonus: Option<ItemCountType>,

    pub idle: Option<RotatedAnimation>,
    pub in_motion: Option<RotatedAnimation>,
    pub shadow_idle: Option<RotatedAnimation>,
    pub shadow_in_motion: Option<RotatedAnimation>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_cargo: bool,

    #[serde(flatten)]
    child: T,
    // not implemented
    // pub destroy_action: Option<Trigger>,
    // pub charging_sound: Option<InterruptibleSound>,
}

impl<T: super::Renderable> Deref for RobotWithLogisticInterfaceData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: super::Renderable> super::Renderable for RobotWithLogisticInterfaceData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(
        &self,
        options: &super::RenderOpts,
    ) -> Vec<(MapPosition, Direction)> {
        self.child.heat_buffer_connections(options)
    }
}

/// [`Prototypes/ConstructionRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstructionRobotPrototype.html)
pub type ConstructionRobotPrototype = RobotWithLogisticInterfacePrototype<ConstructionRobotData>;

/// [`Prototypes/ConstructionRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstructionRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstructionRobotData {
    pub construction_vector: Vector,
    pub working: Option<RotatedAnimation>,
    pub shadow_working: Option<RotatedAnimation>,
    pub smoke: Option<Animation>,
    pub sparks: Option<AnimationVariations>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub mined_sound_volume_modifier: f64,

    pub working_light: Option<LightDefinition>,
    // not implemented
    // pub reparing_sound: Option<Sound>,
}

impl super::Renderable for ConstructionRobotData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }
}

/// [`Prototypes/LogisticRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/LogisticRobotPrototype.html)
pub type LogisticRobotPrototype = RobotWithLogisticInterfacePrototype<LogisticRobotData>;

/// [`Prototypes/LogisticRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/LogisticRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LogisticRobotData {
    pub idle_with_cargo: Option<RotatedAnimation>,
    pub in_motion_with_cargo: Option<RotatedAnimation>,
    pub shadow_idle_with_cargo: Option<RotatedAnimation>,
    pub shadow_in_motion_with_cargo: Option<RotatedAnimation>,
}

impl super::Renderable for LogisticRobotData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        None
    }
}
