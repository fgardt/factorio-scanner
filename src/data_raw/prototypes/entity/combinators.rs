use serde::{Deserialize, Serialize};

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/CombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/CombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct CombinatorPrototype<T> {
    pub energy_source: AnyEnergySource, // theoretically only electric and void are valid
    pub active_energy_usage: Energy,
    pub sprites: Sprite4Way,
    pub activity_led_sprites: Sprite4Way,
    pub input_connection_bounding_box: BoundingBox,
    pub output_connection_bounding_box: BoundingBox,
    pub activity_led_light_offsets: (Vector, Vector, Vector, Vector),
    pub screen_light_offsets: (Vector, Vector, Vector, Vector),
    pub input_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),
    pub output_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),

    pub activity_led_light: Option<LightDefinition>,
    pub screen_light: Option<LightDefinition>,

    #[serde(
        default = "helper::u8_5",
        skip_serializing_if = "helper::is_5_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub activity_led_hold_time: u8,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    #[serde(flatten)]
    pub child: T,
}

/// [`Prototypes/ArithmeticCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ArithmeticCombinatorPrototype.html)
pub type ArithmeticCombinatorPrototype = CombinatorPrototype<ArithmeticCombinatorData>;

/// [`Prototypes/ArithmeticCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ArithmeticCombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ArithmeticCombinatorData {
    pub plus_symbol_sprites: Sprite4Way,
    pub minus_symbol_sprites: Sprite4Way,
    pub multiply_symbol_sprites: Sprite4Way,
    pub divide_symbol_sprites: Sprite4Way,
    pub modulo_symbol_sprites: Sprite4Way,
    pub power_symbol_sprites: Sprite4Way,
    pub left_shift_symbol_sprites: Sprite4Way,
    pub right_shift_symbol_sprites: Sprite4Way,
    pub and_symbol_sprites: Sprite4Way,
    pub or_symbol_sprites: Sprite4Way,
    pub xor_symbol_sprites: Sprite4Way,
}

/// [`Prototypes/DeciderCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/DeciderCombinatorPrototype.html)
pub type DeciderCombinatorPrototype = CombinatorPrototype<DeciderCombinatorData>;

/// [`Prototypes/DeciderCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/DeciderCombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct DeciderCombinatorData {
    pub equal_symbol_sprites: Sprite4Way,
    pub greater_symbol_sprites: Sprite4Way,
    pub less_symbol_sprites: Sprite4Way,
    pub not_equal_symbol_sprites: Sprite4Way,
    pub greater_or_equal_symbol_sprites: Sprite4Way,
    pub less_or_equal_symbol_sprites: Sprite4Way,
}

/// [`Prototypes/ConstantCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstantCombinatorPrototype.html)
pub type ConstantCombinatorPrototype = EntityWithOwnerPrototype<ConstantCombinatorData>;

/// [`Prototypes/ConstantCombinatorPrototype`](https://lua-api.factorio.com/latest/prototypes/ConstantCombinatorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstantCombinatorData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub item_slot_count: u32,

    pub sprites: Sprite4Way,
    pub activity_led_sprites: Sprite4Way,
    pub activity_led_light_offsets: (Vector, Vector, Vector, Vector),
    pub circuit_wire_connection_points: (
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    ),

    pub activity_led_light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default, skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default, skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,
}
