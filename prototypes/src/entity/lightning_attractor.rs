use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/LightningAttractorPrototype`](https://lua-api.factorio.com/latest/prototypes/LightningAttractorPrototype.html)
pub type LightningAttractorPrototype = EntityWithOwnerPrototype<LightningAttractorData>;

/// [`Prototypes/LightningAttractorPrototype`](https://lua-api.factorio.com/latest/prototypes/LightningAttractorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LightningAttractorData {
    pub chargable_graphics: Option<ChargableGraphics>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub lightning_strike_offset: MapPosition,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub efficiency: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub range_elongation: f64,

    pub energy_source: Option<ElectricEnergySource>,
}

impl super::Renderable for LightningAttractorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.chargable_graphics.as_ref().and_then(|cg| {
            cg.picture.as_ref().and_then(|p| {
                p.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            })
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}
