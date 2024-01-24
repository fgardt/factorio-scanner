use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/LampPrototype`](https://lua-api.factorio.com/latest/prototypes/LampPrototype.html)
pub type LampPrototype = EntityWithOwnerPrototype<LampData>;

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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub glow_size: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub glow_render_mode: GlowRenderMode,

    #[serde(flatten)]
    pub wire_connection_data: WireConnectionData,
}

impl super::Renderable for LampData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture_off.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        if options.circuit_connected {
            let orientation = options.orientation.unwrap_or_default();
            if let Some(c) = self.wire_connection_data.render_connector(
                orientation,
                render_layers.scale(),
                used_mods,
                image_cache,
            ) {
                render_layers.add_entity(c, &options.position);
            }
        }

        Some(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignalColorMapping {
    #[serde(flatten)]
    pub signal: SignalIDConnector,

    pub color: Color,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GlowRenderMode {
    #[default]
    Additive,
    Multiplicative,
}
