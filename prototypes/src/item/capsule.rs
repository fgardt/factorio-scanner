use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// [`Prototypes/CapsulePrototype`](https://lua-api.factorio.com/latest/prototypes/CapsulePrototype.html)
pub type CapsulePrototype = crate::BasePrototype<CapsulePrototypeData>;

/// [`Prototypes/CapsulePrototype`](https://lua-api.factorio.com/latest/prototypes/CapsulePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CapsulePrototypeData {
    pub capsule_action: types::CapsuleAction,
    pub radius_color: Option<types::Color>,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for CapsulePrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
