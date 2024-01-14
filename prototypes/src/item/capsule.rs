use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use types::{CapsuleAction, Color};

/// [`Prototypes/CapsulePrototype`](https://lua-api.factorio.com/latest/prototypes/CapsulePrototype.html)
pub type CapsulePrototype = crate::BasePrototype<CapsulePrototypeData>;

/// [`Prototypes/CapsulePrototype`](https://lua-api.factorio.com/latest/prototypes/CapsulePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CapsulePrototypeData {
    pub capsule_action: CapsuleAction,
    pub radius_color: Option<Color>,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for CapsulePrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
