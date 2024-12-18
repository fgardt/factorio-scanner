use serde::{Deserialize, Serialize};
use types::HeatBuffer;

use super::Renderable;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeatBufferEntityData<T: Renderable> {
    pub heat_buffer: HeatBuffer,

    #[serde(flatten)]
    child: T,
}

impl<T: Renderable> std::ops::Deref for HeatBufferEntityData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for HeatBufferEntityData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut types::ImageCache,
    ) -> super::RenderOutput {
        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.fluid_box_connections(options)
    }

    // TODO: mirroring
    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        let mut res = self.heat_buffer.connection_points();
        res.append(&mut self.child.heat_buffer_connections(options));
        res
    }

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }
}
