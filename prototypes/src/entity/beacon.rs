use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/BeaconPrototype`](https://lua-api.factorio.com/latest/prototypes/BeaconPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconPrototype(EntityWithOwnerPrototype<BeaconData>);

impl Deref for BeaconPrototype {
    type Target = EntityWithOwnerPrototype<BeaconData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BeaconPrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for BeaconPrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, used_mods, image_cache)
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
    pub base_picture: Option<Animation>,
    pub radius_visualisation_picture: Option<Sprite>,
    pub allowed_effects: Option<EffectTypeLimitation>,
}

impl super::Renderable for BeaconData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
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
