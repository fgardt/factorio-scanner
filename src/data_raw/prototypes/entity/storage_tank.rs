use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/StorageTankPrototype`](https://lua-api.factorio.com/latest/prototypes/StorageTankPrototype.html)
pub type StorageTankPrototype = EntityWithOwnerPrototype<StorageTankData>;

/// [`Prototypes/StorageTankPrototype`](https://lua-api.factorio.com/latest/prototypes/StorageTankPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageTankData {
    pub fluid_box: FluidBox,
    pub window_bounding_box: BoundingBox,
    pub pictures: StorageTankPictures,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub flow_length_in_ticks: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub two_direction_only: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_wire_connection_points: Option<(
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    )>,
    pub circuit_connector_sprites: Option<(
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
    )>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub scale_info_icons: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageTankPictures {
    pub picture: Sprite4Way,
    pub window_background: Sprite,
    pub fluid_background: Sprite,
    pub flow_sprite: Sprite,
    pub gas_flow: Animation,
}
