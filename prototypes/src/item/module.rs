use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;
use types::{Color, Effect, ModuleCategoryID};

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

    pub art_style: Option<String>,

    pub beacon_tint: Option<BeaconVisualizationTints>,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for ModulePrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/ModulePrototype/BeaconVisualizationTints`](https://lua-api.factorio.com/latest/prototypes/ModulePrototype.html#beacon_tint)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BeaconVisualizationTints {
    pub primary: Option<Color>,
    pub secondary: Option<Color>,
    pub tertiary: Option<Color>,
    pub quaternary: Option<Color>,
}
