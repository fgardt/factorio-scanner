use serde::{Deserialize, Serialize};

use types::AttackParameters;

/// [`Prototypes/GunPrototype`](https://lua-api.factorio.com/latest/prototypes/GunPrototype.html)
pub type GunPrototype = crate::BasePrototype<GunPrototypeData>;

/// [`Prototypes/GunPrototype`](https://lua-api.factorio.com/latest/prototypes/GunPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct GunPrototypeData {
    pub attack_parameters: AttackParameters,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for GunPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
