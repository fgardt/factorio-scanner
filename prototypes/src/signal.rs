use serde::{Deserialize, Serialize};

use types::{Icon, RenderableGraphics, VirtualSignalID};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/VirtualSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/VirtualSignalPrototype.html)
pub type VirtualSignalPrototype = crate::BasePrototype<VirtualSignalPrototypeData>;

/// [`Prototypes/VirtualSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/VirtualSignalPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct VirtualSignalPrototypeData {
    #[serde(flatten)]
    pub icon: Icon,
}

impl VirtualSignalPrototypeData {
    pub fn get_icon(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut types::ImageCache,
    ) -> Option<types::GraphicsOutput> {
        self.icon.render(scale, used_mods, image_cache, &())
    }
}

namespace_struct! {
    AllTypes,
    VirtualSignalID,
    "virtual-signal"
}
