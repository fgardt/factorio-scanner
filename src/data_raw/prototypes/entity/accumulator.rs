use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/AccumulatorPrototype`](https://lua-api.factorio.com/latest/prototypes/AccumulatorPrototype.html)
pub type AccumulatorPrototype = EntityWithOwnerPrototype<AccumulatorData>;

/// [`Prototypes/AccumulatorPrototype`](https://lua-api.factorio.com/latest/prototypes/AccumulatorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AccumulatorData {
    pub energy_source: ElectricEnergySource,
    pub picture: Sprite,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub charge_cooldown: u16,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub discharge_cooldown: u16,

    pub charge_animation: Option<Animation>,
    pub charge_light: Option<LightDefinition>,
    pub discharge_animation: Option<Animation>,
    pub discharge_light: Option<LightDefinition>,

    pub circuit_wire_connection_point: Option<WireConnectionPoint>,
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,
    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub default_output_signal: Option<SignalIDConnector>,
}
