use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/LampPrototype`](https://lua-api.factorio.com/latest/prototypes/LampPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct LampPrototype(EntityWithOwnerPrototype<LampData>);

impl Deref for LampPrototype {
    type Target = EntityWithOwnerPrototype<LampData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LampPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for LampPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, used_mods, image_cache)
    }
}

/// [`Prototypes/LampPrototype`](https://lua-api.factorio.com/latest/prototypes/LampPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LampData {
    pub picture_on: Sprite,
    pub picture_off: Sprite,
    pub energy_usage_per_tick: Energy,
    pub energy_source: AnyEnergySource, // theoretically limited to electric / void source

    pub light: Option<LightDefinition>,
    pub light_when_colored: Option<LightDefinition>,
    pub circuit_wire_connection_point: Option<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_connector_sprites: Option<CircuitConnectorSprites>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub glow_size: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub glow_color_intensity: f64,

    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub darkness_for_all_lamps_on: f64,

    #[serde(default = "helper::f64_03", skip_serializing_if = "helper::is_03_f64")]
    pub darkness_for_all_lamps_off: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub always_on: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signal_to_color_mapping: FactorioArray<SignalColorMapping>,

    // TODO: skip serializing if default
    #[serde(default)]
    pub glow_render_mode: GlowRenderMode,
}

impl super::Renderable for LampData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.picture_off
            .render(used_mods, image_cache, &options.into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignalColorMapping {
    #[serde(flatten)]
    pub signal: SignalIDConnector,

    pub color: Color,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GlowRenderMode {
    #[default]
    Additive,
    Multiplicative,
}
