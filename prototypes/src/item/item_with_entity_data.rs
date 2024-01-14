use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use types::{FactorioArray, FileName, IconData};

/// [`Prototypes/ItemWithEntityDataPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithEntityDataPrototype.html)
pub type ItemWithEntityDataPrototype = crate::BasePrototype<ItemWithEntityDataPrototypeData>;

/// [`Prototypes/ItemWithEntityDataPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithEntityDataPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemWithEntityDataPrototypeData {
    pub icon_tintable: Option<FileName>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icon_tintables: FactorioArray<IconData>,

    pub icon_tintable_mask: Option<FileName>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icon_tintable_masks: FactorioArray<IconData>,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for ItemWithEntityDataPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
