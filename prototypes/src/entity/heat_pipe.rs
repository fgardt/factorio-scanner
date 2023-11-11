use serde::{Deserialize, Serialize};

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/HeatPipePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatPipePrototype.html)
pub type HeatPipePrototype = EntityWithOwnerPrototype<HeatPipeData>;

/// [`Prototypes/HeatPipePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatPipeData {
    pub connection_sprites: ConnectableEntityGraphics,
    pub heat_glow_sprites: ConnectableEntityGraphics,
    pub heat_buffer: HeatBuffer,
}

impl super::Renderable for HeatPipeData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.connection_sprites
            .get(options.connections.unwrap_or_default())
            .render(used_mods, image_cache, &options.into())
    }
}
