use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/BurnerGeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/BurnerGeneratorPrototype.html)
pub type BurnerGeneratorPrototype = EntityWithOwnerPrototype<BurnerGeneratorData>;

/// [`Prototypes/BurnerGeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/BurnerGeneratorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BurnerGeneratorData {
    pub energy_source: ElectricEnergySource,
    pub burner: BurnerEnergySource,
    pub animation: Option<Animation4Way>,
    pub max_power_output: Energy,

    pub idle_animation: Option<Animation4Way>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub always_draw_idle_animation: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub perceived_performance: PerceivedPerformance,
}

impl super::Renderable for BurnerGeneratorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.animation.as_ref().and_then(|a| {
            a.render(
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
