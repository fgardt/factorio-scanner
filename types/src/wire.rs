use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use crate::{FactorioArray, LightDefinition, Sprite, Vector};

/// [`Types/WirePosition`](https://lua-api.factorio.com/latest/types/WirePosition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WirePosition {
    pub copper: Option<Vector>,
    pub green: Option<Vector>,
    pub red: Option<Vector>,
}

/// [`Types/WireConnectionPoint`](https://lua-api.factorio.com/latest/types/WireConnectionPoint.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct WireConnectionPoint {
    pub wire: WirePosition,
    pub shadow: WirePosition,
}

/// [`Types/CircuitConnectorSprites`](https://lua-api.factorio.com/latest/types/CircuitConnectorSprites.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitConnectorSprites {
    pub led_red: Sprite,
    pub led_green: Sprite,
    pub led_blue: Sprite,
    pub led_light: LightDefinition,

    pub connector_main: Option<Sprite>,
    pub connector_shadow: Option<Sprite>,

    pub wire_pins: Option<Sprite>,
    pub wire_pins_shadow: Option<Sprite>,

    pub led_blue_off: Option<Sprite>,
    pub led_blue_light_offset: Option<Vector>,
    pub red_green_led_light_offset: Option<Vector>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WireDrawFlags {
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WireConnectionData {
    PowerPole {
        connection_points: FactorioArray<WireConnectionPoint>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        maximum_wire_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    PowerSwitch {
        left_wire_connection_point: Box<WireConnectionPoint>,
        right_wire_connection_point: Box<WireConnectionPoint>,
        circuit_wire_connection_point: Box<WireConnectionPoint>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    Combinator {
        input_connection_points: Box<[WireConnectionPoint; 4]>,
        output_connection_points: Box<[WireConnectionPoint; 4]>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    Single {
        circuit_wire_connection_point: Option<Box<WireConnectionPoint>>,
        circuit_connector_sprites: Option<Box<CircuitConnectorSprites>>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    OrientedCardinal {
        circuit_wire_connection_point: Option<Box<[WireConnectionPoint; 4]>>,
        circuit_connector_sprites: Option<Box<[CircuitConnectorSprites; 4]>>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
    OrientedAny {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        circuit_wire_connection_points: FactorioArray<WireConnectionPoint>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        circuit_connector_sprites: FactorioArray<CircuitConnectorSprites>,

        #[serde(default, skip_serializing_if = "helper::is_default")]
        circuit_wire_max_distance: f64,

        #[serde(flatten)]
        draw_flags: WireDrawFlags,
    },
}
