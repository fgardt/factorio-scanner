use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{Effect, FactorioArray, ModuleCategoryID, RecipeID};

/// [`Prototypes/ModulePrototype`](https://lua-api.factorio.com/latest/prototypes/ModulePrototype.html)
pub type ModulePrototype = crate::BasePrototype<ModulePrototypeData>;

/// [`Prototypes/ModulePrototype`](https://lua-api.factorio.com/latest/prototypes/ModulePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ModulePrototypeData {
    pub category: ModuleCategoryID,
    pub tier: u32,
    pub effect: Effect,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub requires_beacon_alt_mode: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitation: FactorioArray<RecipeID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitation_blacklist: FactorioArray<RecipeID>,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for ModulePrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
