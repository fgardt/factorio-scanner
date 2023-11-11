use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
pub type HeatInterfacePrototype = EntityWithOwnerPrototype<HeatInterfaceData>;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatInterfaceData {
    pub heat_buffer: HeatBuffer,

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
    ) -> Option<GraphicsOutput> {
        self.picture
            .as_ref()?
            .render(used_mods, image_cache, &options.into())
    }
}
