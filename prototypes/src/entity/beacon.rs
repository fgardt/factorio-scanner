use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/BeaconPrototype`](https://lua-api.factorio.com/latest/prototypes/BeaconPrototype.html)
pub type BeaconPrototype = EntityWithOwnerPrototype<BeaconData>;

/// [`Prototypes/BeaconPrototype`](https://lua-api.factorio.com/latest/prototypes/BeaconPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconData {
    pub energy_usage: Energy,
    pub energy_source: AnyEnergySource, // must be electric or void
    pub supply_area_distance: f64,
    pub distribution_effectivity: f64,
    pub module_specification: ModuleSpecification,

    pub graphics_set: Option<BeaconGraphicsSet>,
    pub animation: Option<Animation>,
    pub base_picture: Option<Animation>,
    pub radius_visualisation_picture: Option<Sprite>,
    pub allowed_effects: Option<EffectTypeLimitation>,
}

impl super::Renderable for BeaconData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        if let Some(set) = self.graphics_set.as_ref() {
            set.render(used_mods, image_cache, &options.into())
        } else {
            self.base_picture
                .as_ref()
                .and_then(|g| g.render(used_mods, image_cache, &options.into()))
        }
    }
}
