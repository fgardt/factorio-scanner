use serde::{Deserialize, Serialize};

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;

/// [`Prototypes/HeatPipePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatPipePrototype(EntityWithOwnerPrototype<HeatPipeData>);

impl super::Renderable for HeatPipePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/HeatPipePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatPipeData {
    pub connection_sprites: ConnectableEntityGraphics,
    pub heat_glow_sprites: ConnectableEntityGraphics,
    pub heat_buffer: HeatBuffer,
}

impl super::Renderable for HeatPipeData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.connection_sprites
            .get(options.connections.unwrap_or_default())
            .render(options.factorio_dir, &options.used_mods, &options.into())
    }
}
