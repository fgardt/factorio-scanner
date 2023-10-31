use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatInterfacePrototype(EntityWithOwnerPrototype<HeatInterfaceData>);

impl super::Renderable for HeatInterfacePrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, image_cache)
    }
}

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
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.picture.as_ref()?.render(
            options.factorio_dir,
            &options.used_mods,
            image_cache,
            &options.into(),
        )
    }
}
