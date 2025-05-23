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

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        let mut res = self.heat_buffer.heat_buffer_connections(options);
        res.append(&mut self.child.heat_buffer_connections(options));
        res
    }

    fn recipe_visible(&self) -> bool {
        self.child.recipe_visible()
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        self.heat_buffer
            .render_debug(options, used_mods, render_layers);
        self.child.render_debug(options, used_mods, render_layers);
    }
}

impl Renderable for HeatBuffer {
    fn render(
        &self,
        options: &crate::entity::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut types::ImageCache,
    ) -> crate::entity::RenderOutput {
        // TODO: render heat buffer connections

        Some(())
    }

    fn heat_buffer_connections(
        &self,
        options: &crate::entity::RenderOpts,
    ) -> Vec<types::MapPosition> {
        // TODO: mirroring

        self.connection_points()
    }

    fn render_debug(
        &self,
        options: &crate::entity::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        const ORANGE: [u8; 4] = [255, 155, 0, 255];

        for conn in &self.connections {
            let pos = conn.position + options.position;
            render_layers.draw_dot(&pos, ORANGE);
            render_layers.draw_direction(&pos, conn.direction, ORANGE);
        }
    }
}
