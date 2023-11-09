use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/HeatInterfacePrototype`](https://lua-api.factorio.com/latest/prototypes/HeatInterfacePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct HeatInterfacePrototype(EntityWithOwnerPrototype<HeatInterfaceData>);

impl Deref for HeatInterfacePrototype {
    type Target = EntityWithOwnerPrototype<HeatInterfaceData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeatInterfacePrototype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl super::Renderable for HeatInterfacePrototype {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.0.render(options, used_mods, image_cache)
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
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
    ) -> Option<GraphicsOutput> {
        self.picture
            .as_ref()?
            .render(used_mods, image_cache, &options.into())
    }
}
