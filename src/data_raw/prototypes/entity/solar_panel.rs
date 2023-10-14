use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;

/// [`Prototypes/SolarPanelPrototype`](https://lua-api.factorio.com/latest/prototypes/SolarPanelPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SolarPanelPrototype(EntityWithOwnerPrototype<SolarPanelData>);

impl super::Renderable for SolarPanelPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/SolarPanelPrototype`](https://lua-api.factorio.com/latest/prototypes/SolarPanelPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SolarPanelData {
    pub energy_source: ElectricEnergySource,
    pub picture: SpriteVariations,
    pub production: Energy,
    pub overlay: Option<SpriteVariations>,
}

impl super::Renderable for SolarPanelData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.picture
            .render(options.factorio_dir, &options.used_mods, &options.into())
    }
}
