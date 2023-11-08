use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ProgrammableSpeakerPrototype`](https://lua-api.factorio.com/latest/prototypes/ProgrammableSpeakerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ProgrammableSpeakerPrototype(EntityWithOwnerPrototype<ProgrammableSpeakerData>);

impl super::Renderable for ProgrammableSpeakerPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, used_mods, image_cache)
    }
}

/// [`Prototypes/ProgrammableSpeakerPrototype`](https://lua-api.factorio.com/latest/prototypes/ProgrammableSpeakerPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammableSpeakerData {
    pub energy_source: AnyEnergySource,
    pub energy_usage_per_tick: Energy,
    pub sprite: Sprite,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub maximum_polyphony: u32,

    pub instruments: FactorioArray<ProgrammableSpeakerInstrument>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub audible_distance_modifier: f64, // docs specify single precision float

    pub circuit_wire_connection_point: Option<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,
}

impl super::Renderable for ProgrammableSpeakerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.sprite.render(used_mods, image_cache, &options.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammableSpeakerInstrument {
    pub name: String,
    pub notes: FactorioArray<ProgrammableSpeakerNote>,
}

// TODO: move this to sound type module
/// [`Types/ProgrammableSpeakerNote`](https://lua-api.factorio.com/latest/types/ProgrammableSpeakerNote.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammableSpeakerNote {
    pub name: String,
    // not implemented
    // pub sound: Sound,
}
