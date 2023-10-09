use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/WallPrototype`](https://lua-api.factorio.com/latest/prototypes/WallPrototype.html)
pub type WallPrototype = EntityWithOwnerPrototype<WallData>;

/// [`Prototypes/WallPrototype`](https://lua-api.factorio.com/latest/prototypes/WallPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WallData {
    pub pictures: WallPictures,

    #[serde(default, skip_serializing_if = "helper::is_0_u32")]
    pub visual_merge_group: u32,

    pub circuit_wire_connection_point: Option<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
    pub default_output_signal: Option<SignalIDConnector>,

    pub wall_diode_green: Option<Sprite4Way>,
    pub wall_diode_red: Option<Sprite4Way>,
    pub wall_diode_green_light_top: Option<LightDefinition>,
    pub wall_diode_green_light_right: Option<LightDefinition>,
    pub wall_diode_green_light_bottom: Option<LightDefinition>,
    pub wall_diode_green_light_left: Option<LightDefinition>,
    pub wall_diode_red_light_top: Option<LightDefinition>,
    pub wall_diode_red_light_right: Option<LightDefinition>,
    pub wall_diode_red_light_bottom: Option<LightDefinition>,
    pub wall_diode_red_light_left: Option<LightDefinition>,

    pub connected_gate_visualization: Option<Sprite>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WallPictures {
    pub single: SpriteVariations,
    pub straight_vertical: SpriteVariations,
    pub straight_horizontal: SpriteVariations,
    pub corner_right_down: SpriteVariations,
    pub corner_left_down: SpriteVariations,
    pub t_up: SpriteVariations,
    pub ending_right: SpriteVariations,
    pub ending_left: SpriteVariations,
    pub filling: Option<SpriteVariations>,
    pub water_connection_patch: Option<Sprite4Way>,
    pub gate_connection_patch: Option<Sprite4Way>,
}
