use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/FlyingRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/FlyingRobotPrototype.html)
pub type FlyingRobotPrototype<T> = EntityWithOwnerPrototype<FlyingRobotData<T>>;

/// [`Prototypes/FlyingRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/FlyingRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FlyingRobotData<T> {
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

/// [`Prototypes/CombatRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/CombatRobotPrototype.html)
pub type CombatRobotPrototype = FlyingRobotPrototype<CombatRobotData>;

/// [`Prototypes/CombatRobotPrototype`](https://lua-api.factorio.com/latest/prototypes/CombatRobotPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CombatRobotData {
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

/// [`Prototypes/RobotWithLogisticInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/RobotWithLogisticInterfacePrototype.html)
pub type RobotWithLogisticInterfacePrototype<T> =
    FlyingRobotPrototype<RobotWithLogisticInterfaceData<T>>;

/// [`Prototypes/RobotWithLogisticInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/RobotWithLogisticInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct RobotWithLogisticInterfaceData<T> {
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
    pub working_light: Option<LightDefinition>,
    // not implemented
    // pub reparing_sound: Option<Sound>,
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
