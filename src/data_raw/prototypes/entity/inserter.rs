use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/InserterPrototype`](https://lua-api.factorio.com/latest/prototypes/InserterPrototype.html)
pub type InserterPrototype = EntityWithOwnerPrototype<InserterData>;

/// [`Prototypes/InserterPrototype`](https://lua-api.factorio.com/latest/prototypes/InserterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct InserterData {
    pub extension_speed: f64,
    pub rotation_speed: f64,
    pub insert_position: Vector,
    pub pickup_position: Vector,

    pub platform_picture: Sprite4Way,
    pub hand_base_picture: Sprite,
    pub hand_open_picture: Sprite,
    pub hand_closed_picture: Sprite,
    pub hand_base_shadow: Sprite,
    pub hand_open_shadow: Sprite,
    pub hand_closed_shadow: Sprite,

    pub energy_source: AnyEnergySource,
    pub energy_per_movement: Option<Energy>,
    pub energy_per_rotation: Option<Energy>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stack: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_custom_vectors: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_burner_leech: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_held_item: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub use_easter_egg: bool, // can the inserter fish or not?

    #[serde(default, skip_serializing_if = "helper::is_0_u8")]
    pub filter_count: u8,

    #[serde(
        default = "helper::f64_075",
        skip_serializing_if = "helper::is_075_f64"
    )]
    pub hand_size: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub default_stack_control_input_signal: Option<SignalIDConnector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_inserter_arrow: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub chase_belt_items: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_u8")]
    pub stack_size_bonus: u8,

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
}
