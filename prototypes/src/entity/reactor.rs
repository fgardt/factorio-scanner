use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ReactorPrototype`](https://lua-api.factorio.com/latest/prototypes/ReactorPrototype.html)
pub type ReactorPrototype = EntityWithOwnerPrototype<ReactorData>;

/// [`Prototypes/ReactorPrototype`](https://lua-api.factorio.com/latest/prototypes/ReactorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactorData {
    pub working_light_picture: Sprite,
    pub heat_buffer: HeatBuffer,
    pub energy_source: AnyEnergySource, // may not be heat energy source
    pub consumption: Energy,

    pub connection_patches_connected: Option<SpriteVariations>,
    pub connection_patches_disconnected: Option<SpriteVariations>,
    pub heat_connection_patches_connected: Option<SpriteVariations>,
    pub heat_connection_patches_disconnected: Option<SpriteVariations>,
    pub lower_layer_picture: Option<Sprite>,
    pub heat_lower_layer_picture: Option<Sprite>,
    pub picture: Option<Sprite>,
    pub light: Option<LightDefinition>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub neighbour_bonus: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub scale_energy_usage: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub use_fuel_glow_color: bool,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub default_fuel_glow_color: Color,
    // not implemented
    // pub meltdown_action: Option<Trigger>,
}

impl super::Renderable for ReactorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        target_size: &TargetSize,
        render_layers: &mut RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        merge_renders(&[
            self.lower_layer_picture
                .as_ref()
                .and_then(|s| s.render(used_mods, image_cache, &options.into())),
            self.picture
                .as_ref()
                .and_then(|s| s.render(used_mods, image_cache, &options.into())),
        ])

        // TODO: include heatpipes (and maybe glow?)
    }
}
