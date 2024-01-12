use serde::{Deserialize, Serialize};

use serde_helper as helper;

/// [`Prototypes/AmmoItemPrototype`](https://lua-api.factorio.com/latest/prototypes/AmmoItemPrototype.html)
pub type AmmoItemPrototype = crate::BasePrototype<AmmoItemPrototypeData>;

/// [`Prototypes/AmmoItemPrototype`](https://lua-api.factorio.com/latest/prototypes/AmmoItemPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct AmmoItemPrototypeData {
    pub ammo_type: AmmoTypeUnion,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub magazine_size: f32,

    #[serde(default, skip_serializing_if = "helper::is_0_f32")]
    pub reload_time: f32,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for AmmoItemPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AmmoTypeUnion {
    Single(types::AmmoType),
    Array(types::FactorioArray<types::AmmoType>),
}
