use serde::{Deserialize, Serialize};

use types::EntityID;

/// [`Prototypes/RailPlannerPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPlannerPrototype.html)
pub type RailPlannerPrototype = crate::BasePrototype<RailPlannerPrototypeData>;

/// [`Prototypes/RailPlannerPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPlannerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RailPlannerPrototypeData {
    pub straight_rail: EntityID,
    pub curved_rail: EntityID,

    #[serde(flatten)]
    parent: super::ItemPrototypeData,
}

impl std::ops::Deref for RailPlannerPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
