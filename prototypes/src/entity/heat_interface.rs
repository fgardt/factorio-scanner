use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, HeatBufferEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
pub type HeatInterfacePrototype = EntityWithOwnerPrototype<HeatBufferEntityData<HeatInterfaceData>>;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatInterfaceData {
    pub picture: Option<Sprite>,

    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,
}

impl super::Renderable for HeatInterfaceData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture.as_ref()?.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }
}
