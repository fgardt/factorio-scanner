use serde::{Deserialize, Serialize};

use serde_helper as helper;

use crate::FactorioArray;

/// [`Types/ItemStackIndex`](https://lua-api.factorio.com/latest/types/ItemStackIndex.html)
pub type ItemStackIndex = u16;

/// [`Types/ItemCountType`](https://lua-api.factorio.com/latest/types/ItemCountType.html)
pub type ItemCountType = u32;

/// [`Types/ItemPrototypeFlags`](https://lua-api.factorio.com/latest/types/ItemPrototypeFlags.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemPrototypeFlag {
    DrawLogisticOverlay,
    ExcludedFromTrashUnrequested,
    AlwaysShow,
    HideFromBonusGui,
    HideFromFuelTooltip,
    NotStackable,
    PrimaryPlaceResult,
    ModOpenable,
    OnlyInCursor,
    Spawnable,
    SpoilResult,
    IgnoreSpoilTimeModifier,
}

/// [`Types/ItemPrototypeFlags`](https://lua-api.factorio.com/latest/types/ItemPrototypeFlags.html)
pub type ItemPrototypeFlags = FactorioArray<ItemPrototypeFlag>;

/// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemProductPrototype {
    Tuple(crate::ItemID, u16),
    Struct(ItemProductPrototypeStruct),
}

/// [`Types/ItemProductPrototype`](https://lua-api.factorio.com/latest/types/ItemProductPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemProductPrototypeStruct {
    pub name: crate::ItemID,

    #[serde(flatten)]
    pub amount: ItemProductAmount,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub catalyst_amount: u16,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub show_details_in_recipe_tooltip: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemProductAmount {
    Static { amount: u16 },
    Range { amount_min: u16, amount_max: u16 },
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FilterMode {
    #[default]
    Whitelist,
    Blacklist,
}
