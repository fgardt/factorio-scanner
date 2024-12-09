use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/LampPrototype`](https://lua-api.factorio.com/latest/prototypes/LampPrototype.html)
pub type LampPrototype = EntityWithOwnerPrototype<WireEntityData<LampData>>;

/// [`Prototypes/LampPrototype`](https://lua-api.factorio.com/latest/prototypes/LampPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LampData {
    pub picture_on: Option<Sprite>,
    pub picture_off: Option<Sprite>,
    pub energy_usage_per_tick: Energy,
    pub energy_source: AnyEnergySource, // theoretically limited to electric / void source

    pub light: Option<LightDefinition>,
    pub light_when_colored: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub glow_size: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub glow_color_intensity: f64,

    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub darkness_for_all_lamps_on: f64,

    #[serde(default = "helper::f64_03", skip_serializing_if = "helper::is_03_f64")]
    pub darkness_for_all_lamps_off: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_on: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signal_to_color_mapping: FactorioArray<SignalColorMapping>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub glow_render_mode: GlowRenderMode,

    pub default_red_signal: Option<SignalIDConnector>,
    pub default_green_signal: Option<SignalIDConnector>,
    pub default_blue_signal: Option<SignalIDConnector>,
    pub default_rgb_signal: Option<SignalIDConnector>,
}

impl super::Renderable for LampData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture_off.as_ref().and_then(|po| {
            po.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

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
