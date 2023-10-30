use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/FlyingRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/FlyingRobotPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct FlyingRobotPrototype<T: super::Renderable>(EntityWithOwnerPrototype<FlyingRobotData<T>>);

impl<T: super::Renderable> super::Renderable for FlyingRobotPrototype<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

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

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub speed_multiplier_when_out_of_energy: f64,

    #[serde(flatten)]
    pub child: T,
}

impl<T: super::Renderable> super::Renderable for FlyingRobotData<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/CombatRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/CombatRobotPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CombatRobotPrototype(FlyingRobotPrototype<CombatRobotData>);

impl super::Renderable for CombatRobotPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/CombatRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/CombatRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CombatRobotData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub time_to_live: u32,

    pub idle: RotatedAnimation,
    pub in_motion: RotatedAnimation,
    pub shadow_idle: RotatedAnimation,
    pub shadow_in_motion: RotatedAnimation,
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub range_from_player: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub friction: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub follows_player: bool,

    pub light: Option<LightDefinition>,
    // not implemented
    // pub attack_parameters: AttackParameters,
    // pub destroy_action: Option<Trigger>,
}

impl super::Renderable for CombatRobotData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RobotWithLogisticInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/RobotWithLogisticInterfacePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RobotWithLogisticInterfacePrototype<T: super::Renderable>(
    FlyingRobotPrototype<RobotWithLogisticInterfaceData<T>>,
);

impl<T: super::Renderable> super::Renderable for RobotWithLogisticInterfacePrototype<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/RobotWithLogisticInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/RobotWithLogisticInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RobotWithLogisticInterfaceData<T: super::Renderable> {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_payload_size: ItemCountType,

    pub cargo_centered: Vector,

    pub idle: Option<RotatedAnimation>,
    pub in_motion: Option<RotatedAnimation>,
    pub shadow_idle: Option<RotatedAnimation>,
    pub shadow_in_motion: Option<RotatedAnimation>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_cargo: bool,

    #[serde(flatten)]
    pub child: T,
    // not implemented
    // pub destroy_action: Option<Trigger>,
}

impl<T: super::Renderable> super::Renderable for RobotWithLogisticInterfaceData<T> {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/ConstructionRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstructionRobotPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstructionRobotPrototype(RobotWithLogisticInterfacePrototype<ConstructionRobotData>);

impl super::Renderable for ConstructionRobotPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/ConstructionRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstructionRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstructionRobotData {
    pub construction_vector: Vector,
    pub working: Option<RotatedAnimation>,
    pub shadow_working: Option<RotatedAnimation>,
    pub smoke: Option<Animation>,
    pub sparks: Option<AnimationVariations>,
    pub working_light: Option<LightDefinition>,
    // not implemented
    // pub reparing_sound: Option<Sound>,
}

impl super::Renderable for ConstructionRobotData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/LogisticRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/LogisticRobotPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LogisticRobotPrototype(RobotWithLogisticInterfacePrototype<LogisticRobotData>);

impl super::Renderable for LogisticRobotPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

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
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}
