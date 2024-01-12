use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

/// [`Prototypes/ToolPrototype`](https://lua-api.factorio.com/latest/prototypes/ToolPrototype.html)
pub type ToolPrototype = crate::BasePrototype<ToolPrototypeData>;

/// [`Prototypes/ToolPrototype`](https://lua-api.factorio.com/latest/prototypes/ToolPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ToolPrototypeData {
    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub durability: f64,

    pub durability_description_key: Option<String>,
    pub durability_description_value: Option<String>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub infinite: bool,

    #[serde(flatten)]
    pub parent: super::ItemPrototypeData,
}

impl std::ops::Deref for ToolPrototypeData {
    type Target = super::ItemPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/ArmorPrototype`](https://lua-api.factorio.com/latest/prototypes/ArmorPrototype.html)
pub type ArmorPrototype = crate::BasePrototype<ArmorPrototypeData>;

/// [`Prototypes/ArmorPrototype`](https://lua-api.factorio.com/latest/prototypes/ArmorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ArmorPrototypeData {
    pub equipment_grid: Option<types::EquipmentGridID>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resistances: types::Resistances,

    pub inventory_size_bonus: Option<types::ItemStackIndex>,

    #[serde(flatten)]
    pub parent: ToolPrototypeData,
}

impl std::ops::Deref for ArmorPrototypeData {
    type Target = ToolPrototypeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Prototypes/MiningToolPrototype`](https://lua-api.factorio.com/latest/prototypes/MiningToolPrototype.html)
/// _deprecated_
pub type MiningToolPrototype = ToolPrototype;

/// [`Prototypes/RepairToolPrototype`](https://lua-api.factorio.com/latest/prototypes/RepairToolPrototype.html)
pub type RepairToolPrototype = crate::BasePrototype<RepairToolPrototypeData>;

/// [`Prototypes/RepairToolPrototype`](https://lua-api.factorio.com/latest/prototypes/RepairToolPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RepairToolPrototypeData {
    pub speed: f32,

    #[serde(flatten)]
    pub parent: ToolPrototypeData,
    // not implemented
    // pub repair_result: Option<Trigger>,
}
