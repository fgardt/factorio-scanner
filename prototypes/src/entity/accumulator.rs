use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

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

/// [`Types/ChargableGraphics`](https://lua-api.factorio.com/latest/types/ChargableGraphics.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ChargableGraphics {
    pub picture: Option<Sprite>,
    pub charge_animation: Option<Animation>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub charge_animation_is_looped: bool,

    pub charge_light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub charge_cooldown: Option<u16>,

    pub discharge_animation: Option<Animation>,
    pub discharge_light: Option<LightDefinition>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub discharge_cooldown: Option<u16>,
}
