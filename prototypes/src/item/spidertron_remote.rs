use serde::{Deserialize, Serialize};

use types::{FactorioArray, FileName, IconData};

/// [`Prototypes/SpidertronRemotePrototype`](https://lua-api.factorio.com/latest/prototypes/SpidertronRemotePrototype.html)
pub type SpidertronRemotePrototype = crate::BasePrototype<SpidertronRemotePrototypeData>;

/// [`Prototypes/SpidertronRemotePrototype`](https://lua-api.factorio.com/latest/prototypes/SpidertronRemotePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SpidertronRemotePrototypeData {
    #[serde(flatten)]
    pub icon_color_indicator_mask: IconColorIndicatorMask,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for SpidertronRemotePrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IconColorIndicatorMask {
    Array {
        icon_color_indicator_masks: FactorioArray<IconData>,
    },
    Single {
        icon_color_indicator_mask: FileName,
    },
}
