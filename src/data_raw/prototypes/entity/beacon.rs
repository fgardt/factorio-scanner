use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;

/// [`Prototypes/BeaconPrototype`](https://lua-api.factorio.com/latest/prototypes/BeaconPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconPrototype(EntityWithOwnerPrototype<BeaconData>);

impl super::Renderable for BeaconPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

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
    pub base_picture: Option<Sprite>,
    pub radius_visualisation_picture: Option<Sprite>,
    pub allowed_effects: Option<EffectTypeLimitation>,
}

impl super::Renderable for BeaconData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.graphics_set.as_ref().map_or_else(
            || {
                self.base_picture.as_ref().and_then(|b| {
                    b.render(options.factorio_dir, &options.used_mods, &options.into())
                })
            },
            |g| g.render(options.factorio_dir, &options.used_mods, &options.into()),
        )
    }
}
