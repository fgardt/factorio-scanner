use serde::{Deserialize, Serialize};

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PowerSwitchPrototype`](https://lua-api.factorio.com/latest/prototypes/PowerSwitchPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerSwitchPrototype(EntityWithOwnerPrototype<PowerSwitchData>);

impl super::Renderable for PowerSwitchPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, used_mods, image_cache)
    }
}

/// [`Prototypes/PowerSwitchPrototype`](https://lua-api.factorio.com/latest/prototypes/PowerSwitchPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerSwitchData {
    pub power_on_animation: Animation,
    pub overlay_start: Animation,
    pub overlay_loop: Animation,
    pub led_on: Sprite,
    pub led_off: Sprite,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub overlay_start_delay: u8,

    pub circuit_wire_connection_point: WireConnectionPoint,
    pub left_wire_connection_point: WireConnectionPoint,
    pub right_wire_connection_point: WireConnectionPoint,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,
}

impl super::Renderable for PowerSwitchData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.power_on_animation
            .render(used_mods, image_cache, &options.into())

        // TODO: render open / closed depending on render option flag
    }
}
