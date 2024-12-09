use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{FactorioArray, FileName, IconData, SpriteSizeType};

/// [`Prototypes/ItemWithEntityDataPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithEntityDataPrototype.html)
pub type ItemWithEntityDataPrototype = crate::BasePrototype<ItemWithEntityDataPrototypeData>;

/// [`Prototypes/ItemWithEntityDataPrototype`](https://lua-api.factorio.com/latest/prototypes/ItemWithEntityDataPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemWithEntityDataPrototypeData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icon_tintable_masks: FactorioArray<IconData>,

    pub icon_tintable_mask: Option<FileName>,

    #[serde(default = "helper::i16_64", skip_serializing_if = "helper::is_64_i16")]
    pub icon_tintable_mask_size: SpriteSizeType,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icon_tintables: FactorioArray<IconData>,

    pub icon_tintable: Option<FileName>,

    #[serde(default = "helper::i16_64", skip_serializing_if = "helper::is_64_i16")]
    pub icon_tintable_size: SpriteSizeType,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for ItemWithEntityDataPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
