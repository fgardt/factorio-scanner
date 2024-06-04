use serde::{Deserialize, Serialize};

use types::{Icon, ItemSubGroupID, RenderableGraphics, VirtualSignalID};

use crate::helper_macro::namespace_struct;

/// [`Prototypes/VirtualSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/VirtualSignalPrototype.html)
pub type VirtualSignalPrototype = crate::BasePrototype<VirtualSignalPrototypeData>;

/// [`Prototypes/VirtualSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/VirtualSignalPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct VirtualSignalPrototypeData {
    #[serde(flatten)]
    pub icon: Icon,

    #[serde(
        default = "default_subgroup",
        skip_serializing_if = "is_default_subgroup"
    )]
    pub subgroup: ItemSubGroupID,
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

fn default_subgroup() -> ItemSubGroupID {
    ItemSubGroupID::new("virtual-signal")
}

fn is_default_subgroup(subgroup: &ItemSubGroupID) -> bool {
    *subgroup == default_subgroup()
}

namespace_struct! {
    AllTypes,
    VirtualSignalID,
    "virtual-signal"
}
