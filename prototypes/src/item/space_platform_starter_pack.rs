use serde::{Deserialize, Serialize};

use serde_helper as helper;
use types::{FactorioArray, SurfaceID};

use crate::recipe::ItemProductPrototype;

/// [`Prototypes/SpacePlatformStarterPackPrototype`](https://lua-api.factorio.com/latest/prototypes/SpacePlatformStarterPackPrototype.html)
pub type SpacePlatformStarterPackPrototype =
    crate::BasePrototype<SpacePlatformStarterPackPrototypeData>;

/// [`Prototypes/SpacePlatformStarterPackPrototype`](https://lua-api.factorio.com/latest/prototypes/SpacePlatformStarterPackPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct SpacePlatformStarterPackPrototypeData {
    // pub trigger: Option<Trigger>,
    pub surface: Option<SurfaceID>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub create_electric_network: bool,

    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub tiles: FactorioArray<SpacePlatformTileDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_items: FactorioArray<ItemProductPrototype>,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for SpacePlatformStarterPackPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
