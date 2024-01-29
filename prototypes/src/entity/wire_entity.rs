use serde::{Deserialize, Serialize};
use types::WireConnectionData;

use super::Renderable;

#[derive(Debug, Serialize, Deserialize)]
pub struct WireEntityData<T: Renderable> {
    #[serde(flatten)]
    pub wire_connection_data: WireConnectionData,

    #[serde(flatten)]
    pub child: T,
}

impl<T: Renderable> std::ops::Deref for WireEntityData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: Renderable> Renderable for WireEntityData<T> {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &crate::UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut crate::ImageCache,
    ) -> super::RenderOutput {
        let res = self
            .child
            .render(options, used_mods, render_layers, image_cache);

        if options.circuit_connected || options.logistic_connected {
            let orientation = options
                .orientation
                .unwrap_or_else(|| options.direction.to_orientation());
            if let Some(c) = self.wire_connection_data.render_connector(
                orientation,
                render_layers.scale(),
                used_mods,
                image_cache,
            ) {
                render_layers.add_entity(c, &options.position);
            }

            if options.circuit_connected {
                if let Some(p) = self.wire_connection_data.render_pins(
                    orientation,
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                ) {
                    render_layers.add_entity(p, &options.position);
                }

                // cache connection point
                if let Some(c) = self.wire_connection_data.get_connection_point(orientation) {
                    render_layers.store_wire_connection_points(options.entity_id, c);
                }
            }
        }

        res
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        self.child.heat_buffer_connections(options)
    }
}
