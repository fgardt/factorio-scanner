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
        self.fluid_box
            .render(options, used_mods, render_layers, image_cache);

        self.child
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        let mut res = self.fluid_box.fluid_box_connections(options);
        res.append(&mut self.child.fluid_box_connections(options));
        res
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.heat_buffer_connections(options)
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
        self.fluid_box
            .render_debug(options, used_mods, render_layers);
        self.child.render_debug(options, used_mods, render_layers);
    }
}

impl Renderable for FluidBox {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut types::ImageCache,
    ) -> crate::entity::RenderOutput {
        // TODO: render pipe covers / connections
        Some(())
    }

    fn fluid_box_connections(
        &self,
        options: &crate::entity::RenderOpts,
    ) -> Vec<types::MapPosition> {
        self.connection_points(options.direction, options.mirrored)
    }

    fn render_debug(
        &self,
        options: &crate::entity::RenderOpts,
        used_mods: &mod_util::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        const CYAN: [u8; 4] = [0, 255, 255, 255];

        for connection in &self.pipe_connections {
            let Some(pos) = connection.position else {
                continue;
            };

            let pos = pos + options.position;
            render_layers.draw_dot(&pos, CYAN);

            if let Some(direction) = connection.direction {
                render_layers.draw_direction(&pos, direction, CYAN);
            }
        }
    }
}
