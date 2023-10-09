use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/GatePrototype`](https://lua-api.factorio.com/latest/prototypes/GatePrototype.html)
pub type GatePrototype = EntityWithOwnerPrototype<GateData>;

/// [`Prototypes/GatePrototype`](https://lua-api.factorio.com/latest/prototypes/GatePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct GateData {
    pub vertical_animation: Animation,
    pub horizontal_animation: Animation,

    pub vertical_rail_base: Animation,
    pub vertical_rail_animation_left: Animation,
    pub vertical_rail_animation_right: Animation,

    pub horizontal_rail_base: Animation,
    pub horizontal_rail_animation_left: Animation,
    pub horizontal_rail_animation_right: Animation,

    pub wall_patch: Animation,

    pub opening_speed: f64, // docs say single precision, so f32. ¯\_(ツ)_/¯
    pub activation_distance: f64,
    pub timeout_to_close: u32,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub fadeout_interval: u32,

    pub opened_collision_mask: Option<CollisionMask>,
}
