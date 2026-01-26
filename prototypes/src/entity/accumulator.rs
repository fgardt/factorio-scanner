use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/AccumulatorPrototype`](https://lua-api.factorio.com/latest/prototypes/AccumulatorPrototype.html)
pub type AccumulatorPrototype = EntityWithOwnerPrototype<WireEntityData<AccumulatorData>>;

/// [`Prototypes/AccumulatorPrototype`](https://lua-api.factorio.com/latest/prototypes/AccumulatorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct AccumulatorData {
    pub energy_source: ElectricEnergySource,
    pub chargable_graphics: Option<ChargableGraphics>,
    pub default_output_signal: Option<SignalIDConnector>,
}

impl super::Entity for AccumulatorData {}

impl super::Renderable for AccumulatorData {
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
