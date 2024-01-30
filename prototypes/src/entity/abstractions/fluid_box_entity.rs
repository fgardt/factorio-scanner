use serde::{Deserialize, Serialize};
use types::FluidBox;

use super::Renderable;

#[derive(Debug, Serialize, Deserialize)]
pub struct FluidBoxEntityData<T: Renderable> {
    pub fluid_box: FluidBox,

    #[serde(flatten)]
    child: T,
}

impl<T: Renderable> std::ops::Deref for FluidBoxEntityData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for FluidBoxEntityData<T> {
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
        let mut res = self.fluid_box.connection_points(options.direction);
        res.append(&mut self.child.fluid_box_connections(options));
        res
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}
