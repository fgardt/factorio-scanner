use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PumpPrototype`](https://lua-api.factorio.com/latest/prototypes/PumpPrototype.html)
pub type PumpPrototype = EntityWithOwnerPrototype<PumpData>;

/// [`Prototypes/PumpPrototype`](https://lua-api.factorio.com/latest/prototypes/PumpPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PumpData {
    pub fluid_box: FluidBox,
    pub energy_source: AnyEnergySource,
    pub energy_usage: Energy,
    pub pumping_speed: f64,
    pub animations: Animation4Way,

    #[serde(
        default = "helper::f64_1_64",
        skip_serializing_if = "helper::is_1_64_f64"
    )]
    pub fluid_wagon_connector_speed: f64,

    #[serde(
        default = "helper::f64_2_32",
        skip_serializing_if = "helper::is_2_32_f64"
    )]
    pub fluid_wagon_connector_alignment_tolerance: f64,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub fluid_wagon_connector_frame_count: u8,

    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,

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

    pub fluid_wagon_connector_graphics: Option<FluidWagonConnectorGraphics>,
}

impl super::Renderable for PumpData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.animations
            .render(used_mods, image_cache, &options.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluidWagonConnectorGraphics {
    pub load_animations: PumpConnectorGraphics,
    pub unload_animations: PumpConnectorGraphics,
}
