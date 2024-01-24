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

        if options.circuit_connected {
            let orientation = options.orientation.unwrap_or_default();
            if let Some(c) = self.wire_connection_data.render_connector(
                orientation,
                render_layers.scale(),
                used_mods,
                image_cache,
            ) {
                render_layers.add_entity(c, &options.position);
            }
        }

        res
    }
}
