use serde::{Deserialize, Serialize};

use types::{EntityID, FactorioArray};

/// [`Prototypes/RailPlannerPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPlannerPrototype.html)
pub type RailPlannerPrototype = crate::BasePrototype<RailPlannerPrototypeData>;

/// [`Prototypes/RailPlannerPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPlannerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RailPlannerPrototypeData {
    pub rails: FactorioArray<EntityID>,
    pub support: Option<EntityID>,
    pub manual_lenth_limit: Option<f64>, // defaults to `8 * 2 + 1.41 + 0.5`

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for RailPlannerPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
